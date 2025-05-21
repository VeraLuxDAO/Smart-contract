use anchor_lang::prelude::*;

use crate::{
    validate_multisig, GlobalState, MultisigState, ProposalStatus, ProposalSubmitted,
    ReentrancyGuard, VeraluxError, PROPOSAL_MAX_DESCRIPTION_LENGTH, PROPOSAL_MAX_VALUES,
    PROPOSAL_SEED,
};

use super::{ProposalIx, ProposalState};

#[derive(Accounts)]
#[instruction()]
pub struct SubmitProposalCtx<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, constraint = !global.paused @ VeraluxError::Paused)]
    pub global: Account<'info, GlobalState>,

    #[account(mut, constraint = multisig.owners[0] == global.admin @ VeraluxError::UnauthorizedMultisig)]
    pub multisig: Account<'info, MultisigState>,

    #[account(
        init,
        payer = user,
        space = 8 + ProposalState::INIT_SPACE,
        seeds = [PROPOSAL_SEED, user.key().as_ref(), global.proposal_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, ProposalState>,

    system_program: Program<'info, System>,
}

impl SubmitProposalCtx<'_> {
    pub fn handler(ctx: Context<SubmitProposalCtx>, ix: ProposalIx) -> Result<()> {
        let proposal_count = ctx.accounts.global.proposal_count;
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global)?;

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        require!(
            ix.description.len() <= PROPOSAL_MAX_DESCRIPTION_LENGTH,
            VeraluxError::DescriptionTooLong
        );

        require!(
            ix.proposal_values.len() <= PROPOSAL_MAX_VALUES,
            VeraluxError::TooManyProposalValues
        );

        let proposal = &mut ctx.accounts.proposal;
        let now = Clock::get()?.unix_timestamp;

        proposal.id = proposal_count;
        proposal.description = ix.description;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.proposal_type = ix.proposal_type;
        proposal.proposal_values = ix.proposal_values;
        proposal.start_time = now;
        proposal.end_time = now
            .checked_add(14 * 86400)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        proposal.execution_time = now
            .checked_add(3 * 86400)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        proposal.status = ProposalStatus::Pending as u8;

        drop(guard);
        ctx.accounts.global.proposal_count = proposal_count
            .checked_add(1)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        emit!(ProposalSubmitted {
            proposal_id: proposal.id,
            proposal_type: proposal.proposal_type,
            description: String::from_utf8_lossy(proposal.description.as_ref()).to_string(),
        });

        Ok(())
    }
}
