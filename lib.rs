use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{AddPluginV1CpiBuilder},
    types::{Plugin, UpdateAuthority},
};

declare_id!("YOUR_PROGRAM_ID");

mod state;
mod instructions;
mod errors;
mod utils;
mod plugin;

use state::*;
use instructions::*;
use errors::*;
use utils::*;
use plugin::*;

#[program]
pub mod super_vault_plugin {
    use super::*;

    // Initialize the vault for a newly minted NFT
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        amount: u64,
    ) -> Result<()> {
        instructions::initialize_vault(ctx, amount)
    }

    // Update the vault's owner when NFT is transferred
    pub fn update_vault_owner(
        ctx: Context<UpdateVaultOwner>,
    ) -> Result<()> {
        instructions::update_vault_owner(ctx)
    }

    // Process royalty distribution and vault management
    pub fn process_royalty(
        ctx: Context<ProcessRoyalty>,
        amount: u64,
    ) -> Result<()> {
        instructions::process_royalty(ctx, amount)
    }

    // Release funds from the vault when NFT returns to collection
    pub fn release_funds(
        ctx: Context<ReleaseFunds>,
    ) -> Result<()> {
        instructions::release_funds(ctx)
    }

    // Attach plugin to a collection
    pub fn attach_plugin(
        ctx: Context<AttachPluginToCollection>,
        plugin_address: Pubkey,
    ) -> Result<()> {
        plugin::attach_plugin_to_collection(ctx, plugin_address)
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    pub collection: Account<'info, BaseCollectionV1>,
    pub asset: Account<'info, BaseAssetV1>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateVaultOwner<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub asset: Account<'info, BaseAssetV1>,
    pub new_owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessRoyalty<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub asset: Account<'info, BaseAssetV1>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub asset: Account<'info, BaseAssetV1>,
    #[account(mut)]
    pub last_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}