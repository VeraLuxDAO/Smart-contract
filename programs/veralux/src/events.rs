use anchor_lang::prelude::*;

#[event]
pub struct GlobalUpdatedEvent {
    pub launch_timestamp: i64,
    pub threshold: u8,
    pub initial_owners: Vec<Pubkey>,
}

#[event]
pub struct StartedPresaleEvent {
    pub started_presale: bool,
}

#[event]
pub struct WhitelistAddedEvent {
    pub address: Pubkey,
    pub total_whitelisted: u8,
}

#[event]
pub struct PresalePurchaseEvent {
    pub buyer: Pubkey,
    pub usdc_amount: u64,
    pub token_amount: u64,
}

#[event]
pub struct ProposalSubmittedEvent {
    pub proposal_id: u32,
    pub proposal_type: u8,
    pub description: String,
}

#[event]
pub struct MultisigConfirmedEvent {
    pub threshold: u8,
    pub owner_count: u8,
}

#[event]
pub struct MultisigPendingEvent {
    pub initiation_time: i64,
    pub threshold: u8,
}

#[event]
pub struct VotingPowerUpdatedEvent {
    pub old_power: u64,
    pub new_power: u64,
}

#[event]
pub struct StakeEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub tier: u8,
}
