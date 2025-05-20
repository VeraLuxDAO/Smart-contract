use anchor_lang::prelude::*;

use crate::{VeraluxError, MULTISIG_SEED};

use super::MultisigState;

#[derive(Accounts)]
pub struct InitMultisigCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + MultisigState::INIT_SPACE,
        seeds = [MULTISIG_SEED],
        bump,
    )]
    pub multisig: Account<'info, MultisigState>,

    system_program: Program<'info, System>,
}

impl InitMultisigCtx<'_> {
    pub fn handler(
        ctx: Context<InitMultisigCtx>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        require!(
            owners.len() >= 2 && owners.len() <= 5,
            VeraluxError::InvalidOwnersCount
        );
        require!(
            threshold >= 2 && threshold <= owners.len() as u8,
            VeraluxError::InvalidThreshold
        );

        let multisig = &mut ctx.accounts.multisig;
        multisig.admin = ctx.accounts.payer.key();
        multisig.owners = owners;
        multisig.threshold = threshold;
        Ok(())
    }
}
