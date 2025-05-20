use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{GlobalState, VeraluxError, PRESALE_SEED, TREASURY_SEED};

use super::PresalePurchase;

#[derive(Accounts)]
pub struct ClaimPresaleCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, constraint = global.presale_active @ VeraluxError::PresaleNotActive)]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [PRESALE_SEED, user.key().as_ref()],
        bump,
        constraint = presale_purchase.user == user.key() @ VeraluxError::InvalidUser
    )]
    pub presale_purchase: Account<'info, PresalePurchase>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ VeraluxError::InvalidUserTokenAccount
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = treasury_token_account.owner == treasury_authority.key() @ VeraluxError::InvalidTreasuryTokenAccount)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA authority will sign for treasury transfers
    #[account(
        seeds = [TREASURY_SEED, global.treasury_wallet.as_ref()],
        bump
    )]
    pub treasury_authority: AccountInfo<'info>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl ClaimPresaleCtx<'_> {
    pub fn handler(ctx: Context<ClaimPresaleCtx>) -> Result<()> {
        let global = &mut ctx.accounts.global;
        let presale = &mut ctx.accounts.presale_purchase;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= global.launch_timestamp, VeraluxError::PresaleEnded);

        let total = presale.total_purchased;
        let vesting = &global.presale_vesting;

        let weeks_passed = ((now - global.launch_timestamp) / 604800) as u8;
        let mut unlocked_pct = vesting.initial_unlock_pct;

        unlocked_pct =
            unlocked_pct.saturating_add(weeks_passed.saturating_mul(vesting.weekly_unlock_pct));
        unlocked_pct = unlocked_pct.min(100);

        let unlocked_amount = total
            .checked_mul(unlocked_pct as u64)
            .ok_or(VeraluxError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        let claimable = unlocked_amount
            .checked_sub(presale.total_claimed)
            .ok_or(VeraluxError::NothingToClaim)?;

        require!(claimable > 0, VeraluxError::NothingToClaim);

        let seeds = &[
            TREASURY_SEED,
            global.treasury_wallet.as_ref(),
            &[ctx.bumps.treasury_authority],
        ];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.treasury_authority.to_account_info(),
                },
                &[seeds],
            ),
            claimable,
        )?;

        presale.total_claimed = presale
            .total_claimed
            .checked_add(claimable)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        Ok(())
    }
}
