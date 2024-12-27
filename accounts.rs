use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    types::{Plugin, UpdateAuthority},
};
use super::state::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {    
    #[account(
        init,
        seeds = [
            b"vault".as_ref(),
            collection.key().as_ref(),
            asset.mint.as_ref(),
            asset.token_account.as_ref(),
            payer.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + 32 + 32 + 32 + 32 + 8 // size of Vault struct
    )]
    pub vault: Account<'info, Vault>,
    pub collection: Account<'info, BaseCollectionV1>,
    pub asset: Account<'info, BaseAssetV1>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateVaultOwner<'info> {
    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.collection.as_ref(),
            vault.mint.as_ref(),
            vault.token_account.as_ref(),
            vault.owner.as_ref()
        ],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    // The new owner of the NFT, we don't need to verify the account type here
    pub new_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProcessRoyalty<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    // account that will receive the creator's share of the royalty
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.collection.as_ref(),
            vault.mint.as_ref(),
            vault.token_account.as_ref(),
            vault.owner.as_ref()
        ],
        bump,
        close = last_owner
    )]
    pub vault: Account<'info, Vault>,
    /// Last known owner of the NFT, funds will be released to this account
    #[account(mut)]
    pub last_owner: AccountInfo<'info>,
    pub collection: Account<'info, BaseCollectionV1>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPluginToAsset<'info> {
    #[account(mut)]
    pub asset: Account<'info, BaseAssetV1>,
    pub update_authority: Signer<'info>,
}

// If you need to add the plugin to the collection instead of or in addition to individual assets
#[derive(Accounts)]
pub struct AddPluginToCollection<'info> {
    #[account(mut)]
    pub collection: Account<'info, BaseCollectionV1>,
    pub update_authority: Signer<'info>,
}

// Note: This uses `Vault` from the state.rs file which you'd need to import or define here if not already done.