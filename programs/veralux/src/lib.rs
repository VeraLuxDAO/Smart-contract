use anchor_lang::prelude::*;

pub mod constraints;
pub mod errors;
pub mod global;
pub mod lp_staking;
pub mod multisig;
pub mod presale;
pub mod staking;

pub use constraints::*;
pub use errors::*;
pub use global::*;
pub use lp_staking::*;
pub use multisig::*;
pub use presale::*;

declare_id!("xikFFfUa48SD9UZcHjRmkaSKuso7WXQXyxRP4t5EEZY");

#[program]
pub mod veralux {
    use super::*;

    pub fn init_global(ctx: Context<InitGlobalCtx>, treasury_wallet: Pubkey) -> Result<()> {
        InitGlobalCtx::handler(ctx, treasury_wallet)
    }

    pub fn update_global(
        ctx: Context<UpdateGlobalCtx>,
        treasury_usdt_account: Option<Pubkey>,
        treasury_wallet: Option<Pubkey>,
    ) -> Result<()> {
        UpdateGlobalCtx::handler(ctx, treasury_usdt_account, treasury_wallet)
    }

    pub fn init_presale(
        ctx: Context<InitPresaleCtx>,
        token_price_in_usdt: u64,
        max_per_wallet: u64,
        total_presale_cap: u64,
        launch_timestamp: i64,
        vesting: PresaleVestingScheduel,
    ) -> Result<()> {
        InitPresaleCtx::handler(
            ctx,
            token_price_in_usdt,
            max_per_wallet,
            total_presale_cap,
            launch_timestamp,
            vesting,
        )
    }

    pub fn update_presale(
        ctx: Context<UpdatePresaleCtx>,
        token_price_in_usdt: Option<u64>,
        max_per_wallet: Option<u64>,
        total_presale_cap: Option<u64>,
        launch_timestamp: Option<i64>,
        new_vesting: Option<PresaleVestingScheduel>,
        presale_active: Option<bool>,
    ) -> Result<()> {
        UpdatePresaleCtx::handler(
            ctx,
            token_price_in_usdt,
            max_per_wallet,
            total_presale_cap,
            launch_timestamp,
            new_vesting,
            presale_active,
        )
    }

    pub fn buy_presale(
        ctx: Context<BuyPresaleCtx>,
        usdt_amount: u64,
        kyc_verified: bool,
    ) -> Result<()> {
        BuyPresaleCtx::handler(ctx, usdt_amount, kyc_verified)
    }

    pub fn claim_presale(ctx: Context<ClaimPresaleCtx>) -> Result<()> {
        ClaimPresaleCtx::handler(ctx)
    }

    pub fn init_multisig(
        ctx: Context<InitMultisigCtx>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        InitMultisigCtx::handler(ctx, owners, threshold)
    }

    pub fn update_multisig(
        ctx: Context<UpdateMultisigCtx>,
        new_owners: Vec<Pubkey>,
        new_threshold: u8,
    ) -> Result<()> {
        UpdateMultisigCtx::handler(ctx, new_owners, new_threshold)
    }
}
