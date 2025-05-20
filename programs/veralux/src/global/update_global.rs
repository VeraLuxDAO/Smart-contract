use anchor_lang::prelude::*;

use crate::VeraluxError;

use super::GlobalState;

#[derive(Accounts)]
pub struct UpdateGlobalCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = global.admin == payer.key() @ VeraluxError::InvalidAdmin)]
    pub global: Account<'info, GlobalState>,
}

impl UpdateGlobalCtx<'_> {
    pub fn handler(
        ctx: Context<UpdateGlobalCtx>,
        treasury_usdt_account: Option<Pubkey>,
        treasury_wallet: Option<Pubkey>,
    ) -> Result<()> {
        let global = &mut ctx.accounts.global;
        if let Some(treasury_wallet) = treasury_wallet {
            global.treasury_wallet = treasury_wallet;
        }

        if let Some(treasury_usdt_account) = treasury_usdt_account {
            global.treasury_usdt_account = treasury_usdt_account;
        }

        Ok(())
    }
}
