use anchor_lang::prelude::*;

#[error_code]
pub enum VeraluxError {
    /// Global errors
    #[msg("Contract is paused")]
    Paused,
    #[msg("Invalid global admin")]
    InvalidAdmin,

    /// Governance errors
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Too many proposal values")]
    TooManyProposalValues,

    /// Utility errors
    #[msg("Reentrancy guard triggered: Operation already in progress")]
    ReentrancyGuardTriggered,

    /// Authorization errors
    #[msg("Unauthorized: Multisig admin is not the first owner")]
    UnauthorizedMultisig,
    #[msg("Unauthorized: Insufficient signers for multisig operation")]
    InsufficientSigners,
    #[msg("Unauthorized: Signer is not a multisig owner")]
    SignerNotOwner,

    /// Limit
    #[msg("Time lock requirement not met")]
    TimeLockNotMet,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Too few owners in multisig")]
    TooFewOwners,

    #[msg("Invalid authority")]
    InvalidAuthority,
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
    
    #[msg("Invalid multisig admin")]
    InvalidMultisigAdmin,

    #[msg("Staking amount is zero")]
    StakingAmountZero,
    #[msg("Invalid staking tier")]
    InvalidStakingTier,
    #[msg("Unauthorized")]
    Unauthorized,
}
