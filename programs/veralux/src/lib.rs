use anchor_lang::prelude::*;

pub mod constraints;
pub mod errors;
pub mod events;
pub mod global;
pub mod governance;
pub mod multisig;
pub mod utils;

pub use constraints::*;
pub use errors::*;
pub use events::*;
pub use global::*;
pub use governance::*;
pub use multisig::*;
pub use utils::*;

declare_id!("FVrTj1gzeyFBMnQsMXTQUYvtXqU8m6cJtdk3VFeSfHf1");

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
}
