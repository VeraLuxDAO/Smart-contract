use anchor_lang::prelude::*;

use crate::{
    validate_multisig, GlobalUpdatedEvent, MultisigState, ReentrancyGuard, VeraluxError,
    GLOBAL_SEED, MULTISIG_SEED,
};

use super::{GlobalIx, GlobalState};

#[derive(Accounts)]
pub struct UpdateGlobalCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_SEED, payer.key().as_ref()],
        bump,
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, payer.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,
}

impl UpdateGlobalCtx<'_> {
    pub fn handler(ctx: Context<UpdateGlobalCtx>, ix: GlobalIx) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global)?;

        require!(ix.threshold >= 2, VeraluxError::InvalidThreshold);
        require!(
            ix.initial_owners.len() >= 3 && ix.initial_owners.len() <= 5,
            VeraluxError::TooFewOwners
        );

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        drop(guard);

        let global = &mut ctx.accounts.global;

        global.launch_timestamp = ix.launch_timestamp;
        global.paused = false;
        global.admin = multisig.key();

        multisig.owners = ix.initial_owners;
        multisig.threshold = ix.threshold;

        emit!(GlobalUpdatedEvent {
            launch_timestamp: global.launch_timestamp,
            threshold: multisig.threshold,
            initial_owners: multisig.owners.clone(),
        });

        Ok(())
    }
}
