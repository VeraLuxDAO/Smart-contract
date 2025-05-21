use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{get_staking_tier, VeraluxError, STAKER_SEED};

use super::Staker;

#[derive(Accounts)]
pub struct StakeCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKER_SEED, user.key().as_ref()],
        bump,
    )]
    pub staker: Account<'info, Staker>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.amount > 0 @ VeraluxError::StakingAmountZero,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl StakeCtx<'_> {
    pub fn handler(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
        let staker = &mut ctx.accounts.staker;
        let now = Clock::get()?.unix_timestamp;
        let time_staked = now
            .checked_sub(staker.last_staked_at)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        let tier = get_staking_tier(amount, time_staked)?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.staking_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        staker.tier = tier;
        staker.staked_amount = amount;
        staker.last_staked_at = now;
        Ok(())
    }
}
