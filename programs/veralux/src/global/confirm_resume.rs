use anchor_lang::prelude::*;

use crate::{
    validate_multisig,
    GlobalState,
    MultisigState,
    PendingResume,
    ReentrancyGuard,
    ResumeEvent,
    VeraluxError,
    MULTISIG_SEED,
    PENDING_RESUME,
};

#[derive(Accounts)]
pub struct ConfirmResumeCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, constraint = global.paused)]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [PENDING_RESUME, user.key().as_ref()],
        bump
    )]
    pub pending_resume: Account<'info, PendingResume>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, user.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,

    system_program: Program<'info, System>,
}

impl ConfirmResumeCtx<'_> {
    pub fn handler(ctx: Context<ConfirmResumeCtx>) -> Result<()> {
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
        let pending_resume = &mut ctx.accounts.pending_resume;
        require!(now >= pending_resume.initiation_time + 24 * 3600, VeraluxError::TimeLockNotMet);
        global.paused = false;
        global.pause_reason = String::new().into();

        emit!(ResumeEvent { timestamp: now });

        Ok(())
    }
}
