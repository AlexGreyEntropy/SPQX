use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{AddPluginV1CpiBuilder},
    types::{Plugin, UpdateAuthority},
};
use crate::{
    state::Vault,
    errors::SuperVaultError,
};

pub fn integrate_plugin_with_collection(
    ctx: Context<IntegrateWithCollection>,
    plugin_address: Pubkey
) -> Result<()> {
    // This function would add the plugin to the collection
    let cpi_accounts = AddPluginV1CpiBuilder::new(&ctx.accounts.collection)
        .with_update_authority(&ctx.accounts.update_authority)
        .with_plugin(Plugin { address: plugin_address });

    cpi_accounts.invoke()?;

    msg!("Plugin added to collection");
    Ok(())
}

pub fn integrate_plugin_with_asset(
    ctx: Context<IntegrateWithAsset>,
    plugin_address: Pubkey
) -> Result<()> {
    // This function would add the plugin to an individual asset
    let cpi_accounts = AddPluginV1CpiBuilder::new(&ctx.accounts.asset)
        .with_update_authority(&ctx.accounts.update_authority)
        .with_plugin(Plugin { address: plugin_address });

    cpi_accounts.invoke()?;

    msg!("Plugin added to asset");
    Ok(())
}

/// This function would be called to initialize a vault for an NFT when it's minted
pub fn initialize_vault_for_nft(ctx: Context<InitializeVaultForNft>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    *vault = Vault::new(
        ctx.accounts.collection.key(),
        ctx.accounts.asset.mint,
        ctx.accounts.asset.token_account,
        ctx.accounts.payer.key(),
    );

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

/// Account structure for integrating with a collection
#[derive(Accounts)]
pub struct IntegrateWithCollection<'info> {
    #[account(mut)]
    pub collection: Account<'info, BaseCollectionV1>,
    pub update_authority: Signer<'info>,
}

/// Account structure for integrating with an asset
#[derive(Accounts)]
pub struct IntegrateWithAsset<'info> {
    #[account(mut)]
    pub asset: Account<'info, BaseAssetV1>,
    pub update_authority: Signer<'info>,
}

/// Account structure for initializing a vault for an NFT
#[derive(Accounts)]
pub struct InitializeVaultForNft<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    pub collection: Account<'info, BaseCollectionV1>,
    pub asset: Account<'info, BaseAssetV1>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Additional helper functions or CPI calls for plugin management would go here