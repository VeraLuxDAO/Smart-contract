use std::collections::HashSet;

use anchor_lang::prelude::*;

use crate::{GlobalState, MultisigState, VeraluxError, STAKING_TIERS, TOKEN_DECIMALS};

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

pub fn get_staking_tier(amount: u64, time_staked: i64) -> Result<u8> {
    if amount < 20_000 * 10u64.pow(TOKEN_DECIMALS as u32) || time_staked < 7 * 86400 {
        return Ok(255);
    }

    for (i, &tier_amt) in STAKING_TIERS.iter().enumerate() {
        if amount > tier_amt {
            return Ok(i as u8);
        }
    }

    Ok(0)
}
