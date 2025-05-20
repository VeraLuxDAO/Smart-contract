use anchor_lang::prelude::*;

use crate::{GlobalState, VeraluxError};

use super::PresaleVestingScheduel;

#[derive(Accounts)]
pub struct InitPresaleCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = global.admin == payer.key() @ VeraluxError::InvalidAdmin)]
    pub global: Account<'info, GlobalState>,

    system_program: Program<'info, System>,
}

impl InitPresaleCtx<'_> {
    pub fn handler(
        ctx: Context<InitPresaleCtx>,
        token_price_in_usdt: u64,
        max_per_wallet: u64,
        total_presale_cap: u64,
        launch_timestamp: i64,
        vesting: PresaleVestingScheduel,
    ) -> Result<()> {
        let global = &mut ctx.accounts.global;

        global.launch_timestamp = launch_timestamp;
        global.token_price_in_usdt = token_price_in_usdt;
        global.max_per_wallet = max_per_wallet;
        global.total_presale_cap = total_presale_cap;
        global.presale_active = true;
        global.presale_total_sold = 0;
        global.presale_vesting = vesting;
        Ok(())
    }
}
