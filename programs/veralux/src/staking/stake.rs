use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{
    calculate_tier,
    calculate_voting_power,
    GlobalState,
    ReentrancyGuard,
    StakeEvent,
    VeraluxError,
    VotingPowerUpdatedEvent,
    STAKER_SEED,
};

use super::Staker;

#[derive(Accounts)]
pub struct StakeCtx<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = !global.presale_active @ VeraluxError::PresaleEnded,
        constraint = !global.paused @ VeraluxError::Paused
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Staker::INIT_SPACE,
        seeds = [STAKER_SEED, user.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ VeraluxError::InvalidUserTokenAthurity,
        constraint = user_token_account.amount > 0 @ VeraluxError::StakingAmountZero,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_token_account.owner == authority.key() @ VeraluxError::InvalidAdminTokenAthurity
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl StakeCtx<'_> {
    pub fn handler(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let global = &mut ctx.accounts.global;
        let staker = &mut ctx.accounts.staker;
        let now = Clock::get()?.unix_timestamp;
        let old_voting_power = if staker.start_time == 0 {
            0
        } else {
            calculate_voting_power(staker, global, now)?
        };

        if staker.start_time == 0 {
            staker.start_time = now;
            staker.last_claim = now;
            staker.amount = amount;
        } else {
            staker.amount = staker.amount
                .checked_add(amount)
                .ok_or(VeraluxError::ArithmeticOverflow)?;
        }

        let time_staked = now
            .checked_sub(staker.start_time)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        staker.tier = calculate_tier(staker.amount, time_staked)?;
        let new_voting_power = calculate_voting_power(staker, global, now)?;
        let old_total = global.total_voting_power;
        global.total_voting_power = old_total
            .checked_sub(old_voting_power)
            .ok_or(VeraluxError::ArithmeticOverflow)?
            .checked_add(new_voting_power)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        emit!(VotingPowerUpdatedEvent {
            old_power: old_total,
            new_power: global.total_voting_power,
        });

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.admin_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }),
            amount
        )?;

        emit!(StakeEvent {
            user: ctx.accounts.user.key(),
            amount,
            tier: staker.tier,
        });
        Ok(())
    }
}
