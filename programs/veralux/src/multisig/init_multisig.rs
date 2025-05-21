use anchor_lang::prelude::*;

use crate::{GlobalState, VeraluxError, MULTISIG_SEED};

use super::MultisigState;

#[derive(Accounts)]
pub struct InitMultisigCtx<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = !global.paused @ VeraluxError::Paused,
        constraint = global.admin == signer.key() @ VeraluxError::InvalidAdmin
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        init,
        payer = signer,
        space = 8 + MultisigState::INIT_SPACE,
        seeds = [MULTISIG_SEED, signer.key().as_ref()],
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
        require!(
            owners[0] == ctx.accounts.signer.key(),
            VeraluxError::InvalidMultisigAdmin
        );
        multisig.owners = owners;
        multisig.threshold = threshold;
        Ok(())
    }
}
