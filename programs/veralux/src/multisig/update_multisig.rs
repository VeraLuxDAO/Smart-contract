use anchor_lang::prelude::*;

use crate::{validate_multisig, VeraluxError, MULTISIG_SEED};

use super::MultisigState;

#[derive(Accounts)]
pub struct UpdateMultisigCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED],
        bump,
        constraint = multisig.admin == payer.key() @ VeraluxError::InvalidMultisigAdmin,
    )]
    pub multisig: Account<'info, MultisigState>,
}

impl UpdateMultisigCtx<'_> {
    pub fn handler(
        ctx: Context<UpdateMultisigCtx>,
        new_owners: Vec<Pubkey>,
        new_threshold: u8,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        require!(
            new_owners.len() >= 2 && new_owners.len() <= 5,
            VeraluxError::InvalidOwnersCount
        );
        require!(
            new_threshold >= 2 && new_threshold <= new_owners.len() as u8,
            VeraluxError::InvalidThreshold
        );

        multisig.owners = new_owners;
        multisig.threshold = new_threshold;
        Ok(())
    }
}
