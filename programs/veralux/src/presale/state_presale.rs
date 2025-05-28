use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PresalePurchase {
    pub wallet: Pubkey,
    pub total_purchased: u64,
    pub total_private_purchased: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PresaleVesting {
    pub total_amount: u64,
    pub claimed_amount: u64,
}
