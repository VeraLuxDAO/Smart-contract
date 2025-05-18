use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitGlobal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl InitGlobal<'_> {
    pub fn init_global(_ctx: Context<InitGlobal>) -> Result<()> {
        Ok(())
    }
}
