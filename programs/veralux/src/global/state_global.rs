use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub admin: Pubkey,

    // Presale
    pub launch_timestamp: i64,
    pub total_presale_sold: u64,
    pub presale_active: bool,

    // Proposal
    pub proposal_count: u32,

    // Governance
    pub tax_rate: u64,

    #[max_len(100)]
    pub pause_reason: Vec<u8>,
    pub is_processing: bool,
    pub paused: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GlobalIx {
    pub launch_timestamp: i64,
    pub initial_owners: Vec<Pubkey>,
    pub threshold: u8,
}
