use anchor_lang::prelude::*;

use crate::{GlobalState, VeraluxError};

use super::PresaleVestingScheduel;

#[derive(Accounts)]
pub struct UpdatePresaleCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = global.admin == payer.key() @ VeraluxError::InvalidAdmin)]
    pub global: Account<'info, GlobalState>,

    system_program: Program<'info, System>,
}

impl UpdatePresaleCtx<'_> {
    pub fn handler(
        ctx: Context<UpdatePresaleCtx>,
        token_price_in_usdt: Option<u64>,
        max_per_wallet: Option<u64>,
        total_presale_cap: Option<u64>,
        launch_timestamp: Option<i64>,
        new_vesting: Option<PresaleVestingScheduel>,
        presale_active: Option<bool>,
    ) -> Result<()> {
        let global = &mut ctx.accounts.global;

        if let Some(token_price_in_usdt) = token_price_in_usdt {
            global.token_price_in_usdt = token_price_in_usdt;
        }

        if let Some(max_per_wallet) = max_per_wallet {
            global.max_per_wallet = max_per_wallet;
        }

        if let Some(total_presale_cap) = total_presale_cap {
            global.total_presale_cap = total_presale_cap;
        }

        if let Some(vesting) = new_vesting {
            global.presale_vesting = vesting;
        }

        if let Some(active) = presale_active {
            global.presale_active = active;
        }

        if let Some(launch_timestamp) = launch_timestamp {
            global.launch_timestamp = launch_timestamp;
        }

        Ok(())
    }
}
