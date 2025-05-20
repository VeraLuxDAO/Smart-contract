use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::GLOBAL_SEED;

use super::GlobalState;

#[derive(Accounts)]
pub struct InitGlobalCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [GLOBAL_SEED],
        bump,
    )]
    pub global: Account<'info, GlobalState>,

    #[account(mut)]
    pub veralux_token_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
}

impl InitGlobalCtx<'_> {
    pub fn handler(ctx: Context<InitGlobalCtx>, treasury_wallet: Pubkey) -> Result<()> {
        let global = &mut ctx.accounts.global;
        global.admin = ctx.accounts.payer.key();
        global.treasury_wallet = treasury_wallet;

        Ok(())
    }
}
