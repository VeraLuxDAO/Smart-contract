use anchor_lang::prelude::*;

#[error_code]
pub enum VeraluxError {
    /// Global errors
    #[msg("Contract is paused")]
    Paused,
    #[msg("Invalid global admin")]
    InvalidAdmin,
    #[msg("The whitelist list is full")]
    WhitelistFull,
    #[msg("This memeber already in whitelist")]
    AlreadyWhitelisted,
    #[msg("Couldn't update global")]
    AlreadyUpdatedGlobal,

    /// Presale errors
    #[msg("Presale is not active")]
    PresaleNotActive,
    #[msg("Presale purchase amount must be greater than zero.")]
    InvalidPurchaseAmount,

    /// Governance errors
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Too many proposal values")]
    TooManyProposalValues,
    #[msg("Proposal has already been executed")]
    ProposalAlreadyExecuted,
    #[msg("Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("Notice period for proposal execution not met")]
    NoticePeriodNotMet,

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
    #[msg("Unauthorized: Buyer is not a account owner")]
    BuyerNotOwner,
    #[msg("Unauthorized: USDT account owner is not a admin")]
    NotAdminUSDTOwner,

    /// Limit
    #[msg("Time lock requirement not met")]
    TimeLockNotMet,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Too few owners in multisig")]
    TooFewOwners,
    #[msg("Not enough usdt amount in buyer")]
    NotEnoughUSDTBuyer,
    #[msg("Presale supply exceeded")]
    PresaleSupplyExceeded,
    #[msg("Presale maximum per wallet exceeded")]
    PresaleMaxPerWalletExceeded,

    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Presale already initialized")]
    PresaleAlreadyInitialized,
    #[msg("Invalid treasury")]
    InvalidTreasury,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
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
