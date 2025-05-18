use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateGlobal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl UpdateGlobal<'_> {
    pub fn update_global(_ctx: Context<UpdateGlobal>) -> Result<()> {
        Ok(())
    }
}
