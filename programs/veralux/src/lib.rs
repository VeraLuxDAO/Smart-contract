use anchor_lang::prelude::*;

pub mod constraints;
pub mod errors;
pub mod global;
pub mod lp_staking;
pub mod presale;
pub mod staking;

pub use constraints::*;
pub use errors::*;
pub use global::*;
pub use lp_staking::*;
pub use presale::*;

declare_id!("5GS6eXCiEEgr44phBzwJq3MMVtUvjQ3PGpcR1tceXwEj");

#[program]
pub mod veralux {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
