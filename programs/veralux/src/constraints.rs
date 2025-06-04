pub const GLOBAL_SEED: &[u8] = b"global-authority";
pub const PRESALE_PURCHASE: &[u8] = b"presale-purchase-authority";
pub const PRESALE_VESTING: &[u8] = b"presale-vesting-authority";
pub const MULTISIG_SEED: &[u8] = b"multisig-authority";
pub const TREASURY_SEED: &[u8] = b"treasury-authority";
pub const PENDING_MULTISIG_SEED: &[u8] = b"pending-multisig-authority";
pub const STAKER_SEED: &[u8] = b"staker-authority";
pub const LP_STAKER_SEED: &[u8] = b"lp-staker-authority";
pub const PROPOSAL_SEED: &[u8] = b"proposal-authority";
pub const TXN_RECORD: &[u8] = b"txn-record-authority";
pub const PENDING_PAUSE: &[u8] = b"pending-pause-authority";
pub const PENDING_RESUME: &[u8] = b"pending-resume-authority";

pub const TOKEN_DECIMALS: u8 = 9;

pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * (10u64).pow(TOKEN_DECIMALS as u32);

// Presale
pub const PUBLIC_PRESALE_PRICE_PER_TOKEN: u64 = 1600; // 0.0016 USDC
pub const PRIVATE_PRESALE_PRICE_PER_TOKEN: u64 = 1120; // 0.00112 USDC
pub const PRESALE_MAX_PER_WALLET: u64 = 2_000_000 * (10u64).pow(TOKEN_DECIMALS as u32); // 2M tokens
pub const PUBLIC_PRESALE_SUPPLY: u64 = 150_000_000 * (10u64).pow(TOKEN_DECIMALS as u32); // 200M tokens
pub const PRIVATE_PRESALE_SUPPLY: u64 = 150_000_000 * (10u64).pow(TOKEN_DECIMALS as u32); // 100M toknes
pub const WHITELIST_MAX_PER_WALLET: u64 = 1_000_000 * (10u64).pow(TOKEN_DECIMALS as u32); // 1M tokens

pub const TREASURY_RESERVE: u64 = 660_000_000 * (10u64).pow(TOKEN_DECIMALS as u32); // 660M tokens

pub const STAKING_POOL_PCT: u8 = 30;
pub const AIRDROP_POOL_PCT: u8 = 8;
pub const GOVERNANCE_RESERVE_PCT: u8 = 16;
pub const MARKETING_FUND_PCT: u8 = 18;
pub const EMERGENCY_FUND_PCT: u8 = 5;
pub const TEAM_POOL_PCT: u8 = 18;

// Staking
pub const STAKING_TIERS: [u64; 4] = [
    20_000 * (10u64).pow(TOKEN_DECIMALS as u32),
    100_000 * (10u64).pow(TOKEN_DECIMALS as u32),
    500_000 * (10u64).pow(TOKEN_DECIMALS as u32),
    5_000_000 * (10u64).pow(TOKEN_DECIMALS as u32),
];
pub const STAKING_DURATIONS: [i64; 4] = [7 * 86400, 14 * 86400, 30 * 86400, 30 * 86400];
pub const STAKING_REWARDS: [u64; 4] = [
    500 * (10u64).pow(TOKEN_DECIMALS as u32),
    2_500 * (10u64).pow(TOKEN_DECIMALS as u32),
    12_500 * (10u64).pow(TOKEN_DECIMALS as u32),
    125_000 * (10u64).pow(TOKEN_DECIMALS as u32),
];
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

// Limit
pub const PENDING_MULTISIG_PERIOD: u32 = 24 * 3600; // 24h
pub const PROPOSAL_END_PERIOD: u32 = 14 * 86400; // 14 day
pub const PROPOSAL_EXECUTION_PERIOD: u32 = 3 * 86400; // 3 day

pub const TXN_COOLDOWN: i64 = 60;

// Transactionlimits
pub const MAX_SELL_TXN_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const DAILY_SELL_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const MAX_TRANSFER_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const DAILY_TRANSFER_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const PROGRESSIVE_TAX_THRESHOLD: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const TRANSACTION_TRACKING_THRESHOLD: u64 = TOTAL_SUPPLY / 1000; // 0.1%
