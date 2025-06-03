use anchor_lang::prelude::*;

use crate::{
    validate_multisig, GlobalUpdatedEvent, MultisigState, ReentrancyGuard, StartedPresaleEvent,
    UpdateLaunchTimeEvent, VeraluxError, WhitelistAddedEvent, MULTISIG_SEED,
};

use super::{GlobalIx, GlobalState};

#[derive(Accounts)]
pub struct UpdateGlobalCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = global.admin == multisig.key() @ VeraluxError::InvalidAdmin
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED, payer.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,
}

impl UpdateGlobalCtx<'_> {
    pub fn handler(ctx: Context<UpdateGlobalCtx>, ix: GlobalIx) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global)?;

        let multisig = &mut ctx.accounts.multisig;

        require!(
            multisig.owners.len() == 0,
            VeraluxError::AlreadyUpdatedGlobal
        );
        require!(ix.threshold >= 2, VeraluxError::InvalidThreshold);
        require!(
            ix.initial_owners.len() >= 3 && ix.initial_owners.len() <= 5,
            VeraluxError::TooFewOwners
        );

        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        drop(guard);

        let global = &mut ctx.accounts.global;

        global.launch_timestamp = ix.launch_timestamp;
        global.paused = false;
        global.team_wallet = ix.team_wallet;
        global.treasury_wallet = ix.treasury_wallet;
        global.lp_wallet = ix.lp_wallet;
        global.charity_wallet = ix.charity_wallet;

        multisig.owners = ix.initial_owners;
        multisig.threshold = ix.threshold;

        emit!(GlobalUpdatedEvent {
            launch_timestamp: global.launch_timestamp,
            threshold: multisig.threshold,
            initial_owners: multisig.owners.clone(),
        });

        Ok(())
    }

    pub fn start_presale(ctx: Context<UpdateGlobalCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        drop(guard);

        ctx.accounts.global.presale_active = true;

        emit!(StartedPresaleEvent {
            started_presale: ctx.accounts.global.presale_active
        });

        Ok(())
    }

    pub fn stop_presale(ctx: Context<UpdateGlobalCtx>) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        drop(guard);

        let global = &mut ctx.accounts.global;

        global.presale_active = false;

        emit!(StartedPresaleEvent {
            started_presale: global.presale_active
        });

        Ok(())
    }

    pub fn add_whitelist(ctx: Context<UpdateGlobalCtx>, whitelist: Pubkey) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        drop(guard);

        let global = &mut ctx.accounts.global;

        require!(global.whitelist.len() < 50, VeraluxError::WhitelistFull);
        require!(
            !global.whitelist.contains(&whitelist),
            VeraluxError::AlreadyWhitelisted
        );

        global.whitelist.push(whitelist);

        emit!(WhitelistAddedEvent {
            address: whitelist,
            total_whitelisted: global.whitelist.len() as u8
        });

        Ok(())
    }

    pub fn update_launch_time(ctx: Context<UpdateGlobalCtx>, new_time_stamp: i64) -> Result<()> {
        let guard = ReentrancyGuard::new(&mut ctx.accounts.global);

        let multisig = &mut ctx.accounts.multisig;
        let signer_keys: Vec<Pubkey> = ctx
            .remaining_accounts
            .iter()
            .filter(|acc| acc.is_signer)
            .map(|acc| acc.key())
            .collect();

        validate_multisig(multisig, &signer_keys)?;

        drop(guard);

        let global = &mut ctx.accounts.global;

        require!(!global.presale_active, VeraluxError::PresaleStarted);

        global.launch_timestamp = new_time_stamp;

        emit!(UpdateLaunchTimeEvent {
            launchtime: global.launch_timestamp
        });
        Ok(())
    }
}
