use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProposalState {
    pub id: u32,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
    pub votes_for: u64,
    pub votes_against: u64,
    #[max_len(7)]
    pub proposal_values: Vec<u64>,
    #[max_len(200)]
    pub description: Vec<u8>,
    pub proposal_type: u8,
    pub status: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProposalIx {
    pub proposal_values: Vec<u64>,
    pub description: Vec<u8>,
    pub proposal_type: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
}
