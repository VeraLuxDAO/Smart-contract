use anchor_lang::prelude::*;

use crate::{
    validate_multisig,
    GlobalState,
    MultisigState,
    PauseInitiated,
    PendingPause,
    ReentrancyGuard,
    VeraluxError,
    MULTISIG_SEED,
    PENDING_PAUSE,
};

#[derive(Accounts)]
pub struct InitPauseCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, constraint = !global.paused @ VeraluxError::Paused)]
    pub global: Account<'info, GlobalState>,

    #[account(
        init,
        payer = user,
        space = 8 + PendingPause::INIT_SPACE,
        seeds = [PENDING_PAUSE, user.key().as_ref()],
        bump
    )]
    pub pending_pause: Account<'info, PendingPause>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, user.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,

    system_program: Program<'info, System>,
}

impl InitPauseCtx<'_> {
    pub fn handler(ctx: Context<InitPauseCtx>, reason: String) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;

        let signer_keys: Vec<Pubkey> = ctx.remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;
        drop(guard);

        let pending_pause = &mut ctx.accounts.pending_pause;
        pending_pause.reason = reason.into();
        pending_pause.initiation_time = Clock::get()?.unix_timestamp;

        emit!(PauseInitiated { initiation_time: pending_pause.initiation_time });
        Ok(())
    }
}
