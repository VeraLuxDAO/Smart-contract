use anchor_lang::prelude::*;

use crate::{
    GlobalState, MultisigConfirmedEvent, ReentrancyGuard, VeraluxError, MULTISIG_SEED,
    PENDING_MULTISIG_PERIOD, PENDING_MULTISIG_SEED,
};

use super::{MultisigState, PendingMultisigState};

#[derive(Accounts)]
pub struct ConfirmMultisigCtx<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = global.admin == multisig.key() @ VeraluxError::InvalidAdmin,
        constraint = !global.paused @ VeraluxError::Paused
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, signer.key().as_ref()],
        bump,
        constraint = multisig.owners[0] == signer.key() @ VeraluxError::InvalidMultisigAdmin,
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(
        mut,
        seeds = [PENDING_MULTISIG_SEED, signer.key().as_ref()],
        bump,
        close = signer,
    )]
    pub pending_multisig: Account<'info, PendingMultisigState>,
}

impl ConfirmMultisigCtx<'_> {
    pub fn handler(ctx: Context<ConfirmMultisigCtx>) -> Result<()> {
        let _guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let pending_multisig = &ctx.accounts.pending_multisig;
        let now = Clock::get()?.unix_timestamp;
        require!(
            now >= pending_multisig.initiation_time + PENDING_MULTISIG_PERIOD as i64,
            VeraluxError::TimeLockNotMet
        );

        let multisig = &mut ctx.accounts.multisig;
        multisig.owners = pending_multisig.new_owners.clone();
        multisig.threshold = pending_multisig.new_threshold;

        emit!(MultisigConfirmedEvent {
            threshold: multisig.threshold,
            owner_count: multisig.owners.len() as u8
        });

        Ok(())
    }
}
