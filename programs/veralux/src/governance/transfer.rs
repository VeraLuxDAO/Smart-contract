use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::{
    advance_buckets,
    GlobalState,
    ReentrancyGuard,
    TransactionRecord,
    Treasury,
    VeraluxError,
    TREASURY_SEED,
    TXN_COOLDOWN,
    TXN_RECORD,
};

#[derive(Accounts)]
pub struct TransferCtx<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        constraint = !global.presale_active @ VeraluxError::PresaleEnded,
        constraint = !global.paused @ VeraluxError::Paused
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, authority.key().as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + TransactionRecord::INIT_SPACE,
        seeds = [TXN_RECORD, sender.key().as_ref()],
        bump
    )]
    pub txn_record: Account<'info, TransactionRecord>,

    #[account(
        mut,
        constraint = sender_token_account.owner == sender.key(),
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
}

impl TransferCtx<'_> {
    pub fn handler(ctx: Context<TransferCtx>, amount: u64) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let global = &mut ctx.accounts.global;
        let now = Clock::get()?.unix_timestamp;
        let record = &mut ctx.accounts.txn_record;
        let sender_key = ctx.accounts.sender.key();

        require!(
            now.checked_sub(record.last_txn_time).ok_or(VeraluxError::ArithmeticOverflow)? >=
                TXN_COOLDOWN,
            VeraluxError::CooldownActive
        );

        let is_sell = global.dex_prograams
            .iter()
            .any(|dex| {
                *dex == ctx.accounts.recipient_token_account.owner ||
                    dex == ctx.accounts.recipient_token_account.to_account_info().owner
            });

        advance_buckets(record, now)?;
        let bucket_idx = record.current_bucket_index as usize;

        let (current_sell, current_transfer) = if is_sell {
            (record.sell_buckets[bucket_idx], record.transfer_buckets[bucket_idx])
        } else {
            (record.sell_buckets[bucket_idx], record.transfer_buckets[bucket_idx])
        };

        let daily_sell = record.sell_buckets.iter().sum::<u64>();
        let daily_transfer = record.transfer_buckets.iter().sum::<u64>();

        if is_sell {
            require!(amount <= global.max_sell_txn_limit, VeraluxError::MaxSellTxnLimitExceeded);
            require!(
                daily_sell.checked_add(amount).ok_or(VeraluxError::ArithmeticOverflow)? <=
                    global.daily_sell_limit,
                VeraluxError::DailySellLimitExceeded
            );
            record.sell_buckets[bucket_idx] = current_sell
                .checked_add(amount)
                .ok_or(VeraluxError::ArithmeticOverflow)?;
        } else {
            require!(amount <= global.max_transfer_limit, VeraluxError::MaxTransferLimitExceeded);
            require!(
                daily_transfer.checked_add(amount).ok_or(VeraluxError::ArithmeticOverflow)? <=
                    global.daily_transfer_limit,
                VeraluxError::DailyTransferLimitExceeded
            );
            record.transfer_buckets[bucket_idx] = current_transfer
                .checked_add(amount)
                .ok_or(VeraluxError::ArithmeticOverflow)?;
        }

        Ok(())
    }
}
