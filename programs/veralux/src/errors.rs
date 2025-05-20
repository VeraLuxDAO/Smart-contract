use anchor_lang::prelude::*;

#[error_code]
pub enum VeraluxError {
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Invalid admin")]
    InvalidAdmin,
    #[msg("Presale already initialized")]
    PresaleAlreadyInitialized,
    #[msg("Presale not active")]
    PresaleNotActive,
    #[msg("Invalid treasury")]
    InvalidTreasury,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Presale supply exceeded")]
    PresaleSupplyExceeded,
    #[msg("KYC required")]
    KYCRequired,
    #[msg("Invalid user")]
    InvalidUser,
    #[msg("Presale not started")]
    PresaleNotStarted,
    #[msg("Presale already claimed")]
    PresaleAlreadyClaimed,
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Invalid user token account")]
    InvalidUserTokenAccount,
    #[msg("Presale ended")]
    PresaleEnded,
    #[msg("Invalid treasury token account")]
    InvalidTreasuryTokenAccount,
    #[msg("Invalid owners count")]
    InvalidOwnersCount,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Invalid multisig admin")]
    InvalidMultisigAdmin,
    #[msg("Unauthorized: Insufficient signers for multisig operation")]
    InsufficientSigners,
    #[msg("Unauthorized: Signer is not a multisig owner")]
    SignerNotOwner,
}
