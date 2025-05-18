use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateStaking<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl UpdateStaking<'_> {
    pub fn update_staking(_ctx: Context<UpdateStaking>) -> Result<()> {
        Ok(())
    }
}
