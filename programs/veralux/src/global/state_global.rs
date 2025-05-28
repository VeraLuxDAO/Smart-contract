use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub admin: Pubkey,

    // Wallets
    pub admin_wallet: Pubkey,
    pub team_wallet: Pubkey,
    pub treasury_wallet: Pubkey,
    pub lp_wallet: Pubkey,
    pub charity_wallet: Pubkey,

    // Presale
    pub launch_timestamp: i64,
    pub total_public_presale_sold: u64,
    pub total_private_presale_sold: u64,
    pub presale_active: bool,

    // Proposal
    pub proposal_count: u32,

    pub total_boting_power: u64,

    // DAO
    pub tax_rate: u64,

    // Whitelist
    #[max_len(50)]
    pub whitelist: Vec<Pubkey>,

    #[max_len(100)]
    pub pause_reason: Vec<u8>,
    pub is_processing: bool,
    pub paused: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GlobalIx {
    pub team_wallet: Pubkey,
    pub treasury_wallet: Pubkey,
    pub lp_wallet: Pubkey,
    pub charity_wallet: Pubkey,
    pub launch_timestamp: i64,
    pub initial_owners: Vec<Pubkey>,
    pub threshold: u8,
}
