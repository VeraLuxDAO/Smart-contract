use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{
    GlobalState, PresalePurchaseEvent, ReentrancyGuard, VeraluxError, PRESALE_MAX_PER_WALLET,
    PRESALE_PURCHASE, PRESALE_VESTING, PRIVATE_PRESALE_PRICE_PER_TOKEN, PRIVATE_PRESALE_SUPPLY,
    PUBLIC_PRESALE_PRICE_PER_TOKEN, PUBLIC_PRESALE_SUPPLY, TOKEN_DECIMALS,
    WHITELIST_MAX_PER_WALLET,
};

use super::{PresalePurchase, PresaleVesting};

#[derive(Accounts)]
pub struct BuyPresaleCtx<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        constraint = global.presale_active @ VeraluxError::PresaleNotActive,
        constraint = !global.paused @ VeraluxError::Paused
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + PresalePurchase::INIT_SPACE,
        seeds = [PRESALE_PURCHASE, buyer.key().as_ref()],
        bump,
    )]
    pub presale_purchase: Account<'info, PresalePurchase>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + PresaleVesting::INIT_SPACE,
        seeds = [PRESALE_VESTING, buyer.key().as_ref()],
        bump
    )]
    pub presale_vesting: Account<'info, PresaleVesting>,

    #[account(
        mut,
        constraint = buyer_usdc_account.owner == buyer.key() @ VeraluxError::BuyerNotOwner,
        constraint = buyer_usdc_account.amount > 0 @ VeraluxError::NotEnoughUSDTBuyer
    )]
    pub buyer_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_usdc_account.owner == global.admin_wallet @ VeraluxError::NotAdminUSDTOwner
    )]
    pub admin_usdc_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl BuyPresaleCtx<'_> {
    pub fn handler(
        ctx: Context<BuyPresaleCtx>,
        usdc_amount: u64,
        kyc_verified: bool,
    ) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        require!(
            usdc_amount < 1000 * 10u64.pow(6) || kyc_verified,
            VeraluxError::KYCRequired
        );

        let global = &mut ctx.accounts.global;
        let buyer = ctx.accounts.buyer.key();
        let is_whitelisted = global.whitelist.contains(&buyer);

        let purchase = &mut ctx.accounts.presale_purchase;
        let vesting = &mut ctx.accounts.presale_vesting;

        let mut remaining_usdc = usdc_amount;
        let mut private_tokens = 0u64;
        let mut public_tokens = 0u64;

        let already_private = purchase.total_private_purchased;
        let already_total = purchase
            .total_purchased
            .checked_add(already_private)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        require!(
            already_total < PRESALE_MAX_PER_WALLET,
            VeraluxError::PresaleMaxPerWalletExceeded
        );

        if is_whitelisted {
            let private_remaining = WHITELIST_MAX_PER_WALLET.saturating_sub(already_private);
            let private_global_remaining =
                PRIVATE_PRESALE_SUPPLY.saturating_sub(global.total_private_presale_sold);
            let eligible_private_token = private_remaining.min(private_global_remaining);

            let max_private_cost = eligible_private_token
                .checked_mul(PRIVATE_PRESALE_PRICE_PER_TOKEN)
                .unwrap_or(0)
                .checked_div(10u64.pow(TOKEN_DECIMALS as u32))
                .unwrap_or(0);

            let actual_private_usdc = remaining_usdc.min(max_private_cost);

            private_tokens = actual_private_usdc
                .checked_mul(10u64.pow(TOKEN_DECIMALS as u32))
                .unwrap_or(0)
                .checked_div(PRIVATE_PRESALE_PRICE_PER_TOKEN)
                .unwrap_or(0);

            remaining_usdc = remaining_usdc.saturating_sub(actual_private_usdc);
        }

        if remaining_usdc > 0 {
            let public_token_amount = remaining_usdc
                .checked_mul(10u64.pow(TOKEN_DECIMALS as u32))
                .unwrap_or(0)
                .checked_div(PUBLIC_PRESALE_PRICE_PER_TOKEN)
                .unwrap_or(0);

            let public_global_remaining =
                PUBLIC_PRESALE_SUPPLY.saturating_sub(global.total_public_presale_sold);

            let wallet_remaining_cap = PRESALE_MAX_PER_WALLET
                .saturating_sub(already_private)
                .saturating_sub(private_tokens);

            public_tokens = public_token_amount
                .min(public_global_remaining)
                .min(wallet_remaining_cap);

            let public_usdc_used = public_tokens
                .checked_mul(PUBLIC_PRESALE_PRICE_PER_TOKEN)
                .unwrap_or(0)
                .checked_div(10u64.pow(TOKEN_DECIMALS as u32))
                .unwrap_or(0);

            remaining_usdc = remaining_usdc.saturating_sub(public_usdc_used);

            require!(remaining_usdc == 0, VeraluxError::ArithmeticOverflow);
        }

        let total_tokens = private_tokens
            .checked_add(public_tokens)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        require!(total_tokens > 0, VeraluxError::InvalidPurchaseAmount);

        require!(
            already_total.checked_add(total_tokens).unwrap_or(u64::MAX) <= PRESALE_MAX_PER_WALLET,
            VeraluxError::PresaleMaxPerWalletExceeded
        );

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.buyer_usdc_account.to_account_info(),
                    to: ctx.accounts.admin_usdc_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            usdc_amount,
        )?;

        purchase.wallet = ctx.accounts.buyer.key();
        purchase.total_purchased = purchase
            .total_purchased
            .checked_add(public_tokens)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        if is_whitelisted && private_tokens > 0 {
            purchase.total_private_purchased = already_private
                .checked_add(private_tokens)
                .ok_or(VeraluxError::ArithmeticOverflow)?;

            global.total_private_presale_sold = global
                .total_private_presale_sold
                .checked_add(private_tokens)
                .ok_or(VeraluxError::ArithmeticOverflow)?;
        }

        if public_tokens > 0 {
            global.total_public_presale_sold = global
                .total_public_presale_sold
                .checked_add(public_tokens)
                .ok_or(VeraluxError::ArithmeticOverflow)?;
        }

        vesting.total_amount = vesting
            .total_amount
            .checked_add(total_tokens)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        emit!(PresalePurchaseEvent {
            buyer: purchase.wallet,
            usdc_amount,
            token_amount: total_tokens
        });

        Ok(())
    }
}
