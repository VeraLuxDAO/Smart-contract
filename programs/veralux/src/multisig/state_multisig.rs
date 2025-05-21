use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigState {
    #[max_len(5)]
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PendingMultisigState {
    #[max_len(5)]
    pub new_owners: Vec<Pubkey>,
    pub new_threshold: u8,
    pub initiation_time: i64,
}
