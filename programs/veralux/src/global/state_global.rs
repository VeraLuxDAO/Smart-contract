use anchor_lang::prelude::*;

use crate::PresaleVestingScheduel;

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub admin: Pubkey,

    pub launch_timestamp: i64,

    pub presale_total_sold: u64,
    pub token_price_in_usdt: u64,
    pub max_per_wallet: u64,
    pub total_presale_cap: u64,

    pub presale_vesting: PresaleVestingScheduel,
    pub treasury_usdt_account: Pubkey,
    pub treasury_wallet: Pubkey,

    pub presale_active: bool,
}
