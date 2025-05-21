use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Staker {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub last_staked_at: i64,
    pub created_at: i64,
    pub reward_debt: u64,
    pub pending_rewards: u64,
    pub tier: u8,
    pub bump: u8,
}
