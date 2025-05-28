use std::collections::HashSet;

use anchor_lang::prelude::*;

use crate::{GlobalState, MultisigState, Staker, VeraluxError, STAKING_DURATIONS, STAKING_TIERS};

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
        unique_signers.len() >= multisig.threshold as usize,
        VeraluxError::InsufficientSigners
    );

    for signer in unique_signers.iter() {
        require!(
            multisig.owners.contains(signer),
            VeraluxError::SignerNotOwner
        );
    }

    Ok(())
}

fn get_highest_eligible_tier(amount: u64) -> u8 {
    for i in (0..4).rev() {
        if amount >= STAKING_TIERS[i] {
            return i as u8;
        }
    }
    255
}

pub fn calculate_voting_power(staker: &Staker, current_time: i64) -> Result<u64> {
    if staker.tier == 255 {
        return Ok(0);
    }

    let base_voting_power = match staker.tier {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 20,
        _ => return Err(VeraluxError::InvalidTier.into()),
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
    let voting_power = ((base_voting_power as u128 * multiplier as u128) + 999) / 1000;

    let highest_tier = get_highest_eligible_tier(staker.amount);
    if highest_tier == 255 {
        return Ok(0);
    }
    let cap_base = match highest_tier {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 20,
        _ => return Err(VeraluxError::InvalidTier.into()),
    };
    let cap = ((cap_base as u128 * multiplier as u128) + 999) / 1000;
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
