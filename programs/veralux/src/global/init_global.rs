use anchor_lang::prelude::*;

use crate::{
    MultisigState,
    ReentrancyGuard,
    VeraluxError,
    AIRDROP_POOL_PCT,
    EMERGENCY_FUND_PCT,
    GLOBAL_SEED,
    GOVERNANCE_RESERVE_PCT,
    INITIAL_TAX_RATE,
    MARKETING_FUND_PCT,
    MULTISIG_SEED,
    STAKING_POOL_PCT,
    STAKING_REWARDS,
    STAKING_TIERS,
    TEAM_POOL_PCT,
    TREASURY_RESERVE,
    TREASURY_SEED,
};

use super::{ GlobalState, Treasury };

#[derive(Accounts)]
pub struct InitGlobalCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [GLOBAL_SEED, payer.key().as_ref()],
        bump
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        init,
        payer = payer,
        space = 8 + MultisigState::INIT_SPACE,
        seeds = [MULTISIG_SEED, payer.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(
        init,
        payer = payer,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [TREASURY_SEED, payer.key().as_ref()],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    system_program: Program<'info, System>,
}

impl InitGlobalCtx<'_> {
    pub fn handler(ctx: Context<InitGlobalCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);
        drop(guard);

        let global = &mut ctx.accounts.global;
        let multisig = &ctx.accounts.multisig;

        global.admin = multisig.key();
        global.admin_wallet = ctx.accounts.payer.key();
        global.total_public_presale_sold = 0;
        global.total_private_presale_sold = 0;
        global.proposal_count = 0;
        global.paused = true;
        global.pause_reason = Vec::new();
        global.presale_active = false;
        global.tax_rate = INITIAL_TAX_RATE;
        global.is_processing = false;
        global.total_voting_power = 0;
        global.reduction_thresholds = [250, 500, 750];
        global.reduction_factors = [512, 640, 800, 1000];
        global.staking_rewards = STAKING_REWARDS;
        global.staking_tiers = STAKING_TIERS;

        let treasury = &mut ctx.accounts.treasury;
        let total_treasury = TREASURY_RESERVE;
        let staking_pool: u64 = (((total_treasury as u128) * (STAKING_POOL_PCT as u128)) / 1000)
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;
        let airdrop_pool: u64 = (((total_treasury as u128) * (AIRDROP_POOL_PCT as u128)) / 100)
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;
        let governance_reserve: u64 = (
            ((total_treasury as u128) * (GOVERNANCE_RESERVE_PCT as u128)) /
            100
        )
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;
        let marketing_fund: u64 = (((total_treasury as u128) * (MARKETING_FUND_PCT as u128)) / 100)
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;
        let emergency_fund: u64 = (((total_treasury as u128) * (EMERGENCY_FUND_PCT as u128)) / 100)
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;
        let team_pool: u64 = (((total_treasury as u128) * (TEAM_POOL_PCT as u128)) / 100)
            .try_into()
            .map_err(|_| VeraluxError::ArithmeticOverflow)?;
        treasury.staking_pool = staking_pool;
        treasury.airdrop_pool = airdrop_pool;
        treasury.governance_reserve = governance_reserve;
        treasury.marketing_fund = marketing_fund;
        treasury.emergency_fund = emergency_fund;
        treasury.team_pool = team_pool;
        treasury.liquidity_incentive = 0;

        Ok(())
    }
}
