use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdatePresale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl UpdatePresale<'_> {
    pub fn update_presale(_ctx: Context<UpdatePresale>) -> Result<()> {
        Ok(())
    }
}
