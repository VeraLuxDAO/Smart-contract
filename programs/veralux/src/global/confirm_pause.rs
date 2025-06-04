use anchor_lang::prelude::*;

use crate::{
    validate_multisig,
    GlobalState,
    MultisigState,
    PauseEvent,
    PendingPause,
    ReentrancyGuard,
    VeraluxError,
    MULTISIG_SEED,
    PENDING_PAUSE,
};

#[derive(Accounts)]
pub struct ConfirmPauseCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, constraint = !global.paused @ VeraluxError::Paused)]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
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

impl ConfirmPauseCtx<'_> {
    pub fn handler(ctx: Context<ConfirmPauseCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;

        let signer_keys: Vec<Pubkey> = ctx.remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;
        drop(guard);

        let now = Clock::get()?.unix_timestamp;
        let global = &mut ctx.accounts.global;
        let pending_pause = &mut ctx.accounts.pending_pause;
        require!(now >= pending_pause.initiation_time + 24 * 3600, VeraluxError::TimeLockNotMet);
        global.paused = true;
        global.pause_reason = pending_pause.reason.clone();

        emit!(PauseEvent {
            timestamp: now,
            reason: String::from_utf8(global.pause_reason.clone()).unwrap(),
        });

        Ok(())
    }
}
