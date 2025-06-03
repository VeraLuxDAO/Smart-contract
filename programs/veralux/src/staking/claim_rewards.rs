use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{
    get_pending_rewards,
    ClaimRewardsEvent,
    GlobalState,
    NoRewardsEvent,
    ReentrancyGuard,
    Treasury,
    VeraluxError,
    STAKER_SEED,
    STAKING_DURATIONS,
    TREASURY_SEED,
};

use super::Staker;

#[derive(Accounts)]
pub struct ClaimRewardsCtx<'info> {
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
}

impl ClaimRewardsCtx<'_> {
    pub fn handler(ctx: Context<ClaimRewardsCtx>) -> Result<()> {
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

        let rewards = get_pending_rewards(global, staker, treasury, now)?;
        if rewards == 0 {
            emit!(NoRewardsEvent {
                user: ctx.accounts.user.key(),
                reason: "No rewards available".to_string(),
            });
            return Ok(());
        }

        require!(treasury.staking_pool >= rewards, VeraluxError::InsufficientStakingPoolFunds);
        treasury.staking_pool = treasury.staking_pool
            .checked_sub(rewards)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.admin_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }),
            rewards
        )?;

        staker.last_claim = now;

        emit!(ClaimRewardsEvent {
            user: ctx.accounts.user.key(),
            amount: rewards,
        });

        Ok(())
    }
}
