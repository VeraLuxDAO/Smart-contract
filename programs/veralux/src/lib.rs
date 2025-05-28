use anchor_lang::prelude::*;

pub mod constraints;
pub mod errors;
pub mod events;
pub mod global;
pub mod governance;
pub mod multisig;
pub mod presale;
pub mod staking;
pub mod utils;

pub use constraints::*;
pub use errors::*;
pub use events::*;
pub use global::*;
pub use governance::*;
pub use multisig::*;
pub use presale::*;
pub use staking::*;
pub use utils::*;

declare_id!("B7ey9pqVRrtFWDWLWuDr9VxD1VWdn1YviVg74vw4cPwq");

#[program]
pub mod veralux {
    use super::*;

    /// Global
    pub fn init_global(ctx: Context<InitGlobalCtx>) -> Result<()> {
        InitGlobalCtx::handler(ctx)
    }

    pub fn update_global(ctx: Context<UpdateGlobalCtx>, ix: GlobalIx) -> Result<()> {
        UpdateGlobalCtx::handler(ctx, ix)
    }

    pub fn start_presale(ctx: Context<UpdateGlobalCtx>) -> Result<()> {
        UpdateGlobalCtx::start_presale(ctx)
    }

    pub fn add_whitelist(ctx: Context<UpdateGlobalCtx>, whitelist: Pubkey) -> Result<()> {
        UpdateGlobalCtx::add_whitelist(ctx, whitelist)
    }

    /// Preslae
    pub fn buy_presale(
        ctx: Context<BuyPresaleCtx>,
        usdc_amount: u64,
        kyc_verified: bool,
    ) -> Result<()> {
        BuyPresaleCtx::handler(ctx, usdc_amount, kyc_verified)
    }

    /// Multisig
    pub fn init_multisig(
        ctx: Context<InitMultisigCtx>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        InitMultisigCtx::handler(ctx, owners, threshold)
    }

    pub fn confirm_multisig(ctx: Context<ConfirmMultisigCtx>) -> Result<()> {
        ConfirmMultisigCtx::handler(ctx)
    }

    /// Governance
    pub fn submit_proposal(ctx: Context<SubmitProposalCtx>, ix: ProposalIx) -> Result<()> {
        SubmitProposalCtx::handler(ctx, ix)
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposalCtx>) -> Result<()> {
        ExecuteProposalCtx::handler(ctx)
    }
}
