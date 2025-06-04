pub use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LPStaker {
    pub amount: u64,
    pub last_action_time: i64,
    pub unclaimed_rewards: u64,
}
