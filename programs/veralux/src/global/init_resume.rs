use anchor_lang::prelude::*;

use crate::{
    validate_multisig,
    GlobalState,
    MultisigState,
    PendingResume,
    ReentrancyGuard,
    ResumeInitiated,
    MULTISIG_SEED,
    PENDING_RESUME,
};

#[derive(Accounts)]
pub struct InitResumeCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, constraint = global.paused)]
    pub global: Account<'info, GlobalState>,

    #[account(
        init,
        payer = user,
        space = 8 + PendingResume::INIT_SPACE,
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

impl InitResumeCtx<'_> {
    pub fn handler(ctx: Context<InitResumeCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;

        let signer_keys: Vec<Pubkey> = ctx.remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;
        drop(guard);

        let pending_resume = &mut ctx.accounts.pending_resume;
        pending_resume.initiation_time = Clock::get()?.unix_timestamp;

        emit!(ResumeInitiated { initiation_time: pending_resume.initiation_time });
        Ok(())
    }
}
