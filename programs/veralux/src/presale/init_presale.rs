use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitPresale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl InitPresale<'_> {
    pub fn init_presale(_ctx: Context<InitPresale>) -> Result<()> {
        Ok(())
    }
}
