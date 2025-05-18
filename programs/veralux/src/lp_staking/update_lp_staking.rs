use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateLpStaking<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl UpdateLpStaking<'_> {
    pub fn update_lp_staking(_ctx: Context<UpdateLpStaking>) -> Result<()> {
        Ok(())
    }
}
