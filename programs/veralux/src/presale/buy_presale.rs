use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{GlobalState, VeraluxError, PRESALE_SEED};

use super::PresalePurchase;

#[derive(Accounts)]
pub struct BuyPresaleCtx<'info> {
    #[account(mut)]
    pub purchaser: Signer<'info>,

    #[account(mut, constraint = global.presale_active @ VeraluxError::PresaleNotActive)]
    pub global: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = purchaser,
        space = 8 + PresalePurchase::INIT_SPACE,
        seeds = [PRESALE_SEED, purchaser.key().as_ref()],
        bump,
    )]
    pub presale_purchase: Account<'info, PresalePurchase>,

    #[account(mut)]
    pub purchaser_usdt: Account<'info, TokenAccount>,

    #[account(mut, constraint = global.treasury_usdt_account == treasury_usdt.key() @ VeraluxError::InvalidTreasury)]
    pub treasury_usdt: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl BuyPresaleCtx<'_> {
    pub fn handler(
        ctx: Context<BuyPresaleCtx>,
        usdt_amount: u64,
        kyc_verified: bool,
    ) -> Result<()> {
        let global = &mut ctx.accounts.global;
        let presale = &mut ctx.accounts.presale_purchase;
        let now = Clock::get()?.unix_timestamp;

        require!(global.presale_active, VeraluxError::PresaleNotActive);
        require!(now >= global.launch_timestamp, VeraluxError::PresaleEnded);

        require!(global.presale_active, VeraluxError::PresaleNotActive);

        require!(
            usdt_amount < 1000 || kyc_verified,
            VeraluxError::KYCRequired
        );

        let token_price = global.token_price_in_usdt;
        let token_amount = usdt_amount
            .checked_mul(1_000_000)
            .ok_or(VeraluxError::ArithmeticOverflow)?
            .checked_div(token_price)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        let new_total = global
            .presale_total_sold
            .checked_add(token_amount)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        require!(
            new_total <= global.total_presale_cap,
            VeraluxError::PresaleSupplyExceeded
        );

        let user_total = presale
            .total_purchased
            .checked_add(token_amount)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        require!(
            user_total <= global.max_per_wallet,
            VeraluxError::PresaleSupplyExceeded
        );

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.purchaser_usdt.to_account_info(),
                    to: ctx.accounts.treasury_usdt.to_account_info(),
                    authority: ctx.accounts.purchaser.to_account_info(),
                },
            ),
            usdt_amount,
        )?;

        presale.user = ctx.accounts.purchaser.key();
        presale.total_purchased = user_total;
        global.presale_total_sold = new_total;

        Ok(())
    }
}
