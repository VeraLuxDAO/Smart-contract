use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{MultisigState, ReentrancyGuard, GLOBAL_SEED, INITIAL_TAX_RATE, MULTISIG_SEED};

use super::GlobalState;

#[derive(Accounts)]
pub struct InitGlobalCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [GLOBAL_SEED, payer.key().as_ref()],
        bump,
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        init,
        payer = payer,
        space = 8 + MultisigState::INIT_SPACE,
        seeds = [MULTISIG_SEED, payer.key().as_ref()],
        bump,
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(mut)]
    pub veralux_token_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
}

impl InitGlobalCtx<'_> {
    pub fn handler(ctx: Context<InitGlobalCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let global = &mut ctx.accounts.global;

        global.total_presale_sold = 0;
        global.proposal_count = 0;
        global.paused = true;
        global.pause_reason = Vec::new();
        global.presale_active = false;
        global.tax_rate = INITIAL_TAX_RATE;
        global.is_processing = false;

        Ok(())
    }
}
