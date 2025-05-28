use anchor_lang::prelude::*;

use crate::{
    validate_multisig, GlobalState, MultisigState, ProposalStatus, ProposalSubmittedEvent,
    ReentrancyGuard, VeraluxError, MULTISIG_SEED, PROPOSAL_END_PERIOD, PROPOSAL_EXECUTION_PERIOD,
    PROPOSAL_MAX_DESCRIPTION_LENGTH, PROPOSAL_MAX_VALUES, PROPOSAL_SEED,
};

use super::{ProposalIx, ProposalState};

#[derive(Accounts)]
pub struct SubmitProposalCtx<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = global.admin == multisig.key() @ VeraluxError::InvalidAdmin,
        constraint = !global.paused @ VeraluxError::Paused
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, signer.key().as_ref()],
        bump,
        constraint = multisig.owners[0] == signer.key() @ VeraluxError::InvalidMultisigAdmin,
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(
        init,
        payer = signer,
        space = 8 + ProposalState::INIT_SPACE,
        seeds = [PROPOSAL_SEED, signer.key().as_ref(), global.proposal_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, ProposalState>,

    system_program: Program<'info, System>,
}

impl SubmitProposalCtx<'_> {
    pub fn handler(ctx: Context<SubmitProposalCtx>, ix: ProposalIx) -> Result<()> {
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
        drop(guard);

        let global = &mut ctx.accounts.global;
        let proposal = &mut ctx.accounts.proposal;
        let now = Clock::get()?.unix_timestamp;

        msg!("Proposal address: {}", proposal.key());
        msg!("Proposal id: {}", global.proposal_count);
        msg!(
            "Proposal seed (UTF-8): {}",
            String::from_utf8_lossy(PROPOSAL_SEED)
        );
        msg!("Signer address: {}", ctx.accounts.signer.key());

        proposal.id = global.proposal_count;
        proposal.description = ix.description;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.proposal_type = ix.proposal_type;
        proposal.proposal_values = ix.proposal_values;
        proposal.start_time = now;
        proposal.end_time = now
            .checked_add(PROPOSAL_END_PERIOD as i64)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        proposal.execution_time = now
            .checked_add(PROPOSAL_EXECUTION_PERIOD as i64)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
        proposal.status = ProposalStatus::Pending as u8;

        global.proposal_count = proposal
            .id
            .checked_add(1)
            .ok_or(VeraluxError::ArithmeticOverflow)?;

        emit!(ProposalSubmittedEvent {
            proposal_id: proposal.id,
            proposal_type: proposal.proposal_type,
            description: String::from_utf8_lossy(proposal.description.as_ref()).to_string(),
        });

        Ok(())
    }
}
