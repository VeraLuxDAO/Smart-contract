use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PresalePurchase {
    pub user: Pubkey,
    pub total_purchased: u64,
    pub total_claimed: u64,
}
