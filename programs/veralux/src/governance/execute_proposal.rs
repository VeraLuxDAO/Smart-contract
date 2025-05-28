use anchor_lang::prelude::*;

use crate::{GlobalState, ReentrancyGuard, VeraluxError};

use super::ProposalState;

#[derive(Accounts)]
pub struct ExecuteProposalCtx<'info> {
    #[account(
        mut,
        constraint = !global.paused @ VeraluxError::Paused
    )]
    pub global: Account<'info, GlobalState>,

    #[account(mut)]
    pub proposal: Account<'info, ProposalState>,
}

impl ExecuteProposalCtx<'_> {
    pub fn handler(ctx: Context<ExecuteProposalCtx>) -> Result<()> {
        let _guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let now = Clock::get()?.unix_timestamp;
        let proposal = &mut ctx.accounts.proposal;

        require!(proposal.status == 0, VeraluxError::ProposalAlreadyExecuted);
        require!(now > proposal.end_time, VeraluxError::VotingPeriodNotEnded);
        require!(
            now >= proposal.execution_time,
            VeraluxError::NoticePeriodNotMet
        );

        let total_votes = proposal
            .votes_for
            .checked_add(proposal.votes_against)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        Ok(())
    }
}
