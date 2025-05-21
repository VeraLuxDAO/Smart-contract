pub const GLOBAL_SEED: &[u8] = b"global-authority";
pub const PRESALE_SEED: &[u8] = b"presale-authority";
pub const TREASURY_SEED: &[u8] = b"treasury-authority";
pub const MULTISIG_SEED: &[u8] = b"multisig-authority";
pub const PENDING_MULTISIG_SEED: &[u8] = b"pending-multisig-authority";
pub const STAKER_SEED: &[u8] = b"staker-authority";
pub const PROPOSAL_SEED: &[u8] = b"proposal-authority";

pub const TOKEN_DECIMALS: u8 = 9;

// Staking
pub const STAKING_TIERS: [u64; 4] = [20_000, 100_000, 500_000, 5_000_000];
pub const STAKING_LOCKS: [i64; 4] = [7, 14, 30, 30];
pub const STAKING_REWARDS: [u64; 4] = [500, 2_500, 12_500, 125_000];
pub const MULTIPLIER_60D: u64 = 1500;
pub const MULTIPLIER_90D: u64 = 1995;
pub const MULTIPLIER_BASE: u64 = 1000;
pub const MAX_TIERS: usize = 4;

// Governance
// Proposal
pub const PROPOSAL_MAX_VALUES: usize = 7;
pub const PROPOSAL_MAX_DESCRIPTION_LENGTH: usize = 200;

// Global
pub const INITIAL_TAX_RATE: u64 = 500;
