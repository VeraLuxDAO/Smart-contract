use std::collections::HashSet;

use anchor_lang::prelude::*;

pub mod init_multisig;
pub mod state_multisig;
pub mod update_multisig;

pub use init_multisig::*;
pub use state_multisig::*;
pub use update_multisig::*;

use crate::VeraluxError;

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
