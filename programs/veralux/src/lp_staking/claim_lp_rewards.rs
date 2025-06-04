use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{
    ClaimLPRewardsEvent,
    GlobalState,
    LPStaker,
    NoRewardsEvent,
    ReentrancyGuard,
    VeraluxError,
    LP_STAKER_SEED,
};

#[derive(Accounts)]
pub struct ClaimLPRewardsCtx<'info> {
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
        seeds = [LP_STAKER_SEED, user.key().as_ref()],
        bump
    )]
    pub lp_staker: Account<'info, LPStaker>,

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

impl ClaimLPRewardsCtx<'_> {
    pub fn handler(ctx: Context<ClaimLPRewardsCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let now = Clock::get()?.unix_timestamp;
        let lp_staker = &mut ctx.accounts.lp_staker;

        require!(now - lp_staker.last_action_time >= 7 * 86400, VeraluxError::LockPeriodNotMet);
        let rewards = lp_staker.unclaimed_rewards;
        if rewards == 0 {
            emit!(NoRewardsEvent {
                user: ctx.accounts.user.key(),
                reason: "No unclaimed rewards available".to_string(),
            });
            return Ok(());
        }

        lp_staker.unclaimed_rewards = 0;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.admin_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }),
            rewards
        )?;

        emit!(ClaimLPRewardsEvent {
            user: ctx.accounts.user.key(),
            amount: rewards,
        });

        Ok(())
    }
}
