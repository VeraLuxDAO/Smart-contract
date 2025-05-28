use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Staker {
    pub start_time: i64,
    pub last_claim: i64,
    pub amount: u64,
    pub tier: u8,
}
