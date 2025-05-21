use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{VeraluxError, STAKER_SEED};

use super::Staker;

#[derive(Accounts)]
pub struct UnstakeCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKER_SEED, user.key().as_ref()],
        bump,
        constraint = staker.owner == user.key() @ VeraluxError::Unauthorized,
    )]
    pub staker: Account<'info, Staker>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl UnstakeCtx<'_> {
    pub fn handler(ctx: Context<UnstakeCtx>) -> Result<()> {
        let staker = &mut ctx.accounts.staker;
        let now = Clock::get()?.unix_timestamp;

        Ok(())
    }
}
