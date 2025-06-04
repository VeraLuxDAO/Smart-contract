use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{ GlobalState, LPStaker, ReentrancyGuard, UnstakeLPEvent, VeraluxError, LP_STAKER_SEED };

#[derive(Accounts)]
pub struct UnstakeLpCtx<'info> {
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
        constraint = user_lp_token_account.owner == user.key()
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_lp_token_account.owner == authority.key()
    )]
    pub admin_lp_token_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

impl UnstakeLpCtx<'_> {
    pub fn handler(ctx: Context<UnstakeLpCtx>, amount: u64) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let now = Clock::get()?.unix_timestamp;
        let lp_staker = &mut ctx.accounts.lp_staker;
        require!(lp_staker.amount >= amount, VeraluxError::InsufficientStakedAmount);
        require!(now - lp_staker.last_action_time >= 7 * 86400, VeraluxError::LockPeriodNotMet);

        lp_staker.amount = lp_staker.amount
            .checked_sub(amount)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        lp_staker.last_action_time = now;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.admin_lp_token_account.to_account_info(),
                to: ctx.accounts.user_lp_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }),
            amount
        )?;

        if lp_staker.amount == 0 {
            let user_account = ctx.accounts.user.to_account_info();
            let lp_staker_account = lp_staker.to_account_info();
            let rent_lamports = Rent::get()?.minimum_balance(LPStaker::INIT_SPACE);
            let lamports = lp_staker_account.lamports();
            if lamports > rent_lamports {
                **user_account.try_borrow_mut_lamports()? += lamports - rent_lamports;
                **lp_staker_account.try_borrow_mut_lamports()? = rent_lamports;
            }
            lp_staker_account.assign(&anchor_lang::system_program::ID);
            lp_staker_account.realloc(0, false)?;
        }

        emit!(UnstakeLPEvent {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }
}
