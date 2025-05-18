use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitLpStaking<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl InitLpStaking<'_> {
    pub fn init_lp_staking(_ctx: Context<InitLpStaking>) -> Result<()> {
        Ok(())
    }
}
