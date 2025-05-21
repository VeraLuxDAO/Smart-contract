use anchor_lang::prelude::*;

use crate::{validate_multisig, GlobalState, MultisigState, ReentrancyGuard, VeraluxError};

#[derive(Accounts)]
pub struct InitPresaleCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = global.admin == payer.key() @ VeraluxError::InvalidAdmin)]
    pub global: Account<'info, GlobalState>,

    #[account(mut, constraint = multisig.owners[0] == global.admin @ VeraluxError::UnauthorizedMultisig)]
    pub multisig: Account<'info, MultisigState>,
}

impl InitPresaleCtx<'_> {
    pub fn handler(ctx: Context<InitPresaleCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global)?;
        drop(guard);

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();
        validate_multisig(multisig, &signer_keys)?;

        let global = &mut ctx.accounts.global;

        global.presale_active = true;
        Ok(())
    }
}
