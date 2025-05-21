use anchor_lang::prelude::*;

use crate::{Staker, STAKER_SEED};

#[derive(Accounts)]
pub struct InitStakingCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Staker::INIT_SPACE,
        seeds = [STAKER_SEED, user.key().as_ref()],
        bump,
    )]
    pub staker: Account<'info, Staker>,

    system_program: Program<'info, System>,
}

impl InitStakingCtx<'_> {
    pub fn handler(ctx: Context<InitStakingCtx>) -> Result<()> {
        let staker = &mut ctx.accounts.staker;
        let now = Clock::get()?.unix_timestamp;

        staker.owner = ctx.accounts.user.key();
        staker.bump = ctx.bumps.staker;
        staker.tier = 0;
        staker.staked_amount = 0;
        staker.last_staked_at = 0;
        staker.created_at = now;
        staker.reward_debt = 0;
        staker.pending_rewards = 0;

        Ok(())
    }
}
