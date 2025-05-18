use anchor_lang::prelude::*;

#[error_code]
pub enum VeraluxError {
    #[msg("Invalid authority")]
    InvalidAuthority,
}
