use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigState {
    pub admin: Pubkey,
    #[max_len(5)]
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
}
