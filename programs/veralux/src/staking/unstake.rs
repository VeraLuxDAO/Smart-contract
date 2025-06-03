use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{
    calculate_voting_power,
    get_pending_rewards,
    ClaimRewardsEvent,
    GlobalState,
    NoRewardsEvent,
    ReentrancyGuard,
    Treasury,
    UnstakeEvent,
    VeraluxError,
    VotingPowerUpdatedEvent,
    STAKER_SEED,
    STAKING_DURATIONS,
    TREASURY_SEED,
};

use super::Staker;

#[derive(Accounts)]
pub struct UnstakeCtx<'info> {
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
        mut,
        seeds = [STAKER_SEED, user.key().as_ref()],
        bump,
    )]
    pub staker: Account<'info, Staker>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, user.key().as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
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

impl UnstakeCtx<'_> {
    pub fn handler(ctx: Context<UnstakeCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let now = Clock::get()?.unix_timestamp;
        let staker = &mut ctx.accounts.staker;
        let global = &mut ctx.accounts.global;
        let treasury = &mut ctx.accounts.treasury;

        let time_staked = now
            .checked_sub(staker.start_time)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        require!(
            time_staked >= STAKING_DURATIONS[staker.tier as usize],
            VeraluxError::LockPeriodNotMet
        );

        let pending_reward = get_pending_rewards(global, staker, treasury, now)?;
        if pending_reward > 0 {
            require!(
                treasury.staking_pool >= pending_reward,
                VeraluxError::InsufficientStakingPoolFunds
            );
            treasury.staking_pool = treasury.staking_pool
                .checked_sub(pending_reward)
                .ok_or(VeraluxError::ArithmeticOverflow)?;
            token::transfer(
                CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                    from: ctx.accounts.admin_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }),
                pending_reward
            )?;
            emit!(ClaimRewardsEvent {
                user: ctx.accounts.user.key(),
                amount: pending_reward,
            });
            staker.last_claim = now;
        } else {
            emit!(NoRewardsEvent {
                user: ctx.accounts.user.key(),
                reason: "No pending rewards".to_string(),
            });
        }

        let current_voting_power = calculate_voting_power(staker, global, now)?;
        let amount = staker.amount;
        staker.amount = 0;
        staker.tier = 0;
        staker.start_time = 0;
        staker.last_claim = 0;
        let old_total = global.total_voting_power;
        global.total_voting_power = old_total
            .checked_sub(current_voting_power)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        emit!(VotingPowerUpdatedEvent {
            old_power: old_total,
            new_power: global.total_voting_power,
        });

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.admin_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }),
            amount
        )?;

        emit!(UnstakeEvent {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }
}
