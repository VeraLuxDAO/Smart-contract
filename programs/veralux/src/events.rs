use anchor_lang::prelude::*;

#[event]
pub struct ProposalSubmitted {
    pub proposal_id: u64,
    pub proposal_type: u8,
    pub description: String,
}

#[event]
pub struct MultisigUpdatedEvent {
    pub threshold: u8,
    pub owner_count: u8,
}

#[event]
pub struct MultisigPendingEvent {
    pub initiation_time: i64,
    pub threshold: u8,
}

#[event]
pub struct GlobalUpdateEvent {
    pub launch_timestamp: i64,
    pub threshold: u8,
    pub initial_owners: Vec<Pubkey>,
}
