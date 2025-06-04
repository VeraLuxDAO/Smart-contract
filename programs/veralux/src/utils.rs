use std::collections::HashSet;

use anchor_lang::prelude::*;

use crate::{
    GlobalState,
    MultisigState,
    Staker,
    TransactionRecord,
    Treasury,
    VeraluxError,
    STAKING_DURATIONS,
    STAKING_POOL_PCT,
    STAKING_TIERS,
    TREASURY_RESERVE,
};

pub struct ReentrancyGuard<'a> {
    state: &'a mut GlobalState,
}

impl<'a> ReentrancyGuard<'a> {
    pub fn new(state: &'a mut GlobalState) -> Result<Self> {
        require!(!state.is_processing, VeraluxError::ReentrancyGuardTriggered);

        state.is_processing = true;
        Ok(Self { state })
    }
}

impl<'a> Drop for ReentrancyGuard<'a> {
    fn drop(&mut self) {
        self.state.is_processing = false;
    }
}

pub fn validate_multisig(multisig: &MultisigState, signers: &[Pubkey]) -> Result<()> {
    let unique_signers: HashSet<Pubkey> = signers.iter().cloned().collect();

    require!(
        unique_signers.len() >= (multisig.threshold as usize),
        VeraluxError::InsufficientSigners
    );

    for signer in unique_signers.iter() {
        require!(multisig.owners.contains(signer), VeraluxError::SignerNotOwner);
    }

    Ok(())
}

fn get_highest_eligible_tier(global: &GlobalState, amount: u64) -> u8 {
    for i in (0..4).rev() {
        if amount >= global.staking_tiers[i] {
            return i as u8;
        }
    }
    255
}

pub fn calculate_voting_power(
    staker: &Staker,
    global: &GlobalState,
    current_time: i64
) -> Result<u64> {
    if staker.tier == 255 {
        return Ok(0);
    }

    let base_voting_power = match staker.tier {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 20,
        _ => {
            return Err(VeraluxError::InvalidTier.into());
        }
    };

    let time_staked = current_time
        .checked_sub(staker.start_time)
        .ok_or(VeraluxError::ArithmeticOverflow)?;
    let multiplier = if time_staked >= 90 * 86400 {
        1995
    } else if time_staked >= 60 * 86400 {
        1500
    } else {
        1000
    };
    let voting_power = ((base_voting_power as u128) * (multiplier as u128) + 999) / 1000;

    let highest_tier = get_highest_eligible_tier(global, staker.amount);
    if highest_tier == 255 {
        return Ok(0);
    }
    let cap_base = match highest_tier {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 20,
        _ => {
            return Err(VeraluxError::InvalidTier.into());
        }
    };
    let cap = ((cap_base as u128) * (multiplier as u128) + 999) / 1000;
    let final_voting_power = voting_power.min(cap);
    Ok(final_voting_power as u64)
}

pub fn calculate_tier(amount: u64, time_staked: i64) -> Result<u8> {
    if amount < STAKING_TIERS[0] || time_staked < 7 * 86400 {
        return Ok(255);
    }
    for i in (0..4).rev() {
        if amount >= STAKING_TIERS[i] && time_staked >= STAKING_DURATIONS[i] {
            return Ok(i as u8);
        }
    }
    Ok(0)
}

pub fn get_pending_rewards(
    global: &GlobalState,
    staker: &Staker,
    treasury: &Treasury,
    current_time: i64
) -> Result<u64> {
    if staker.amount == 0 || staker.tier == 255 {
        return Ok(0);
    }
    current_time.checked_sub(staker.start_time).ok_or(VeraluxError::ArithmeticOverflow)?;

    let days = (current_time
        .checked_sub(staker.last_claim)
        .ok_or(VeraluxError::ArithmeticOverflow)? / 86400) as u64;
    if days == 0 {
        return Ok(0);
    }
    let pool_fraction = ((treasury.staking_pool as u128) * 1000)
        .checked_div(((TREASURY_RESERVE as u128) * (STAKING_POOL_PCT as u128)) / 100)
        .ok_or(VeraluxError::ArithmeticOverflow)?;

    let reduction_factor = if pool_fraction < (global.reduction_thresholds[0] as u128) {
        global.reduction_factors[0]
    } else if pool_fraction < (global.reduction_thresholds[1] as u128) {
        global.reduction_factors[1]
    } else if pool_fraction < (global.reduction_thresholds[2] as u128) {
        global.reduction_factors[2]
    } else {
        global.reduction_factors[3]
    };

    let base_reward = global.staking_rewards[staker.tier as usize] / 7;
    let reward: u64 = (((base_reward as u128) * (reduction_factor as u128) * (days as u128)) / 1000)
        .try_into()
        .map_err(|_| VeraluxError::ArithmeticOverflow)?;
    Ok(reward)
}

pub fn advance_buckets(record: &mut TransactionRecord, now: i64) -> Result<()> {
    const HOUR_SECONDS: i64 = 3600;
    const DAY_HOURS: usize = 24;

    if record.bucket_start_time == 0 {
        record.bucket_start_time = now - (now % HOUR_SECONDS);
        record.current_bucket_index = 0;
        return Ok(());
    }

    let elapsed_seconds = now
        .checked_sub(record.bucket_start_time)
        .ok_or(VeraluxError::ArithmeticOverflow)?;
    let hours_passed = (elapsed_seconds / HOUR_SECONDS) as u64;

    if hours_passed == 0 {
        return Ok(());
    }

    if hours_passed >= (DAY_HOURS as u64) {
        record.sell_buckets = [0; DAY_HOURS];
        record.transfer_buckets = [0; DAY_HOURS];
        record.current_bucket_index = 0;
        record.bucket_start_time = now - (now % HOUR_SECONDS);
    } else {
        let mut index = record.current_bucket_index as usize;
        let steps = hours_passed as usize;
        for _ in 0..steps {
            index = (index + 1) % DAY_HOURS;
            record.sell_buckets[index] = 0;
            record.transfer_buckets[index] = 0;
        }
        record.current_bucket_index = index as u8;
        record.bucket_start_time = record.bucket_start_time
            .checked_add((hours_passed as i64) * HOUR_SECONDS)
            .ok_or(VeraluxError::ArithmeticOverflow)?;
    }

    Ok(())
}
