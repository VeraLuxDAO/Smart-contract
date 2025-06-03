use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{
    ClaimRewardsEvent, GlobalState, NoRewardsEvent, ReentrancyGuard, VeraluxError, PRESALE_VESTING,
};

use super::PresaleVesting;

#[derive(Accounts)]
pub struct ClaimPresaleCtx<'info> {
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
        seeds = [PRESALE_VESTING, user.key().as_ref()],
        bump,
        constraint = vesting.total_amount > 0 @ VeraluxError::UninitializedAccount
    )]
    pub vesting: Account<'info, PresaleVesting>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key()  @ VeraluxError::InvalidUserTokenAthurity
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_token_account.owner == authority.key()  @ VeraluxError::InvalidAdminTokenAthurity
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

impl ClaimPresaleCtx<'_> {
    pub fn handler(ctx: Context<ClaimPresaleCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let global = &mut ctx.accounts.global;
        let vesting = &mut ctx.accounts.vesting;
        let now = Clock::get()?.unix_timestamp;

        let time_since_launch = now
            .checked_sub(global.launch_timestamp)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        require!(time_since_launch >= 0, VeraluxError::VestingNotStarted);

        let weeks_passed = (time_since_launch / (7 * 86400)) as u64;
        let unlock_percent = 100.min(10 + 10 * weeks_passed);
        let claimable = (vesting.total_amount as u128 * unlock_percent as u128 / 100)
            .checked_sub(vesting.claimed_amount as u128)
            .ok_or(VeraluxError::ArithmeticOverflow)?
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;

        if claimable == 0 {
            emit!(NoRewardsEvent {
                user: ctx.accounts.user.key(),
                reason: "No tokens available to claim".to_string()
            });
            return Ok(());
        }

        vesting.claimed_amount = vesting
            .claimed_amount
            .checked_add(claimable)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.admin_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            claimable,
        )?;

        emit!(ClaimRewardsEvent {
            user: ctx.accounts.user.key(),
            amount: claimable,
        });

        Ok(())
    }
}
