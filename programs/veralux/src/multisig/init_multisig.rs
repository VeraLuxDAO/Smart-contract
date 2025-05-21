use anchor_lang::prelude::*;

use crate::{
    validate_multisig, GlobalState, MultisigPendingEvent, ReentrancyGuard, VeraluxError,
    MULTISIG_SEED, PENDING_MULTISIG_SEED,
};

use super::{MultisigState, PendingMultisigState};

#[derive(Accounts)]
pub struct InitMultisigCtx<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = !global.paused @ VeraluxError::Paused,
        constraint = global.admin == multisig.key() @ VeraluxError::InvalidAdmin
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, signer.key().as_ref()],
        bump,
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + PendingMultisigState::INIT_SPACE,
        seeds = [PENDING_MULTISIG_SEED, signer.key().as_ref()],
        bump
    )]
    pub pending_multisig: Account<'info, PendingMultisigState>,

    system_program: Program<'info, System>,
}

impl InitMultisigCtx<'_> {
    pub fn handler(
        ctx: Context<InitMultisigCtx>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        let _guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        require!(
            owners.len() >= 2 && owners.len() <= 5,
            VeraluxError::InvalidOwnersCount
        );
        require!(
            threshold >= 2 && threshold <= owners.len() as u8,
            VeraluxError::InvalidThreshold
        );

        let pending_multisig = &mut ctx.accounts.pending_multisig;

        pending_multisig.new_owners = owners;
        pending_multisig.new_threshold = threshold;
        pending_multisig.initiation_time = Clock::get()?.unix_timestamp;

        emit!(MultisigPendingEvent {
            initiation_time: pending_multisig.initiation_time,
            threshold: pending_multisig.new_threshold
        });

        Ok(())
    }
}
