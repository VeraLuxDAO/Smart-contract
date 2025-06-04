pub mod constraints;
pub mod errors;
pub mod events;
pub mod global;
pub mod governance;
pub mod multisig;
pub mod presale;
pub mod staking;
pub mod utils;
pub mod lp_staking;
pub mod vesting;

pub use constraints::*;
pub use errors::*;
pub use events::*;
pub use global::*;
pub use governance::*;
pub use multisig::*;
pub use presale::*;
pub use staking::*;
pub use utils::*;
pub use lp_staking::*;
pub use vesting::*;

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

    pub fn stop_presale(ctx: Context<UpdateGlobalCtx>) -> Result<()> {
        UpdateGlobalCtx::stop_presale(ctx)
    }

    pub fn add_whitelist(ctx: Context<UpdateGlobalCtx>, whitelist: Pubkey) -> Result<()> {
        UpdateGlobalCtx::add_whitelist(ctx, whitelist)
    }

    pub fn update_launch_time(ctx: Context<UpdateGlobalCtx>, new_time_stamp: i64) -> Result<()> {
        UpdateGlobalCtx::update_launch_time(ctx, new_time_stamp)
    }

    pub fn init_pause(ctx: Context<InitPauseCtx>, reason: String) -> Result<()> {
        InitPauseCtx::handler(ctx, reason)
    }

    pub fn confirm_pause(ctx: Context<ConfirmPauseCtx>) -> Result<()> {
        ConfirmPauseCtx::handler(ctx)
    }

    pub fn init_resume(ctx: Context<InitResumeCtx>) -> Result<()> {
        InitResumeCtx::handler(ctx)
    }

    pub fn confirm_resume(ctx: Context<ConfirmResumeCtx>) -> Result<()> {
        ConfirmResumeCtx::handler(ctx)
    }

    /// Preslae
    pub fn buy_presale(
        ctx: Context<BuyPresaleCtx>,
        usdc_amount: u64,
        kyc_verified: bool
    ) -> Result<()> {
        BuyPresaleCtx::handler(ctx, usdc_amount, kyc_verified)
    }

    pub fn claim_presale(ctx: Context<ClaimPresaleCtx>) -> Result<()> {
        ClaimPresaleCtx::handler(ctx)
    }

    /// Token Staking
    pub fn stake_token(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
        StakeCtx::handler(ctx, amount)
    }

    pub fn unstake_token(ctx: Context<UnstakeCtx>) -> Result<()> {
        UnstakeCtx::handler(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewardsCtx>) -> Result<()> {
        ClaimRewardsCtx::handler(ctx)
    }

    /// LP Token Staking
    pub fn stake_lp_token(ctx: Context<StakeLPCtx>, amount: u64) -> Result<()> {
        StakeLPCtx::handler(ctx, amount)
    }

    pub fn unstake_lp_token(ctx: Context<UnstakeLpCtx>, amount: u64) -> Result<()> {
        UnstakeLpCtx::handler(ctx, amount)
    }

    pub fn claim_lp_rewards(ctx: Context<ClaimLPRewardsCtx>) -> Result<()> {
        ClaimLPRewardsCtx::handler(ctx)
    }

    /// Multisig
    pub fn init_multisig(
        ctx: Context<InitMultisigCtx>,
        owners: Vec<Pubkey>,
        threshold: u8
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
