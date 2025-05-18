use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitStaking<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl InitStaking<'_> {
    pub fn init_staking(_ctx: Context<InitStaking>) -> Result<()> {
        Ok(())
    }
}
