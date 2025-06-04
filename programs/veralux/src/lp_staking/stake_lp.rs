use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{ GlobalState, ReentrancyGuard, StakeLPEvent, VeraluxError, LP_STAKER_SEED };

use super::LPStaker;

#[derive(Accounts)]
pub struct StakeLPCtx<'info> {
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
        space = LPStaker::INIT_SPACE,
        seeds = [LP_STAKER_SEED, user.key().as_ref()],
        bump
    )]
    pub lp_staker: Account<'info, LPStaker>,

    #[account(
        mut,
        constraint = user_lp_token_account.owner == user.key()
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_lp_token_account.owner == authority.key()
    )]
    pub admin_lp_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl StakeLPCtx<'_> {
    pub fn handler(ctx: Context<StakeLPCtx>, amount: u64) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let now = Clock::get()?.unix_timestamp;
        let lp_staker = &mut ctx.accounts.lp_staker;
        if lp_staker.amount == 0 {
            lp_staker.last_action_time = now;
        }
        lp_staker.amount = lp_staker.amount
            .checked_add(amount)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.user_lp_token_account.to_account_info(),
                to: ctx.accounts.admin_lp_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }),
            amount
        )?;

        emit!(StakeLPEvent {
            user: ctx.accounts.user.key(),
            amount,
        });
        Ok(())
    }
}
