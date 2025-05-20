use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct PresaleVestingScheduel {
    pub initial_unlock_pct: u8,
    pub weekly_unlock_pct: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PresalePurchase {
    pub user: Pubkey,
    pub total_purchased: u64,
    pub total_claimed: u64,
}
