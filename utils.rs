use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};

pub mod utils {
    use super::*;

    /// Finds the Program Derived Address (PDA) for the vault account.
    /// 
    /// The PDA is derived using seeds from the collection, mint, token account, and owner. 
    /// This ensures uniqueness for each NFT's vault.
    pub fn find_vault_pda(
        collection: &Pubkey,
        mint: &Pubkey,
        token_account: &Pubkey,
        owner: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"vault", 
                collection.as_ref(), 
                mint.as_ref(), 
                token_account.as_ref(), 
                owner.as_ref()
            ],
            &crate::ID,
        )
    }

    /// Verifies if the given NFT (represented by its mint address) belongs to the specified collection.
    /// 
    /// This function checks if the NFT's collection matches the given collection address.
    pub fn verify_nft_in_collection(
        asset: &Account<BaseAssetV1>,
        collection: &Account<BaseCollectionV1>,
    ) -> Result<()> {
        if asset.collection != collection.key() {
            return Err(crate::errors::SuperVaultError::NotInCollection.into());
        }
        Ok(())
    }

    /// Calculates the royalty distribution based on the given amount.
    /// 
    /// Returns a tuple where the first element is the amount for the vault (20%) and the second for the creator (80%).
    pub fn calculate_royalty_distribution(amount: u64) -> (u64, u64) {
        let vault_amount = amount * 20 / 100;
        let creator_amount = amount - vault_amount;  // Safe since we know vault_amount <= amount
        (vault_amount, creator_amount)
    }

    /// Checks if the NFT is currently at the collection address.
    /// 
    /// This is used to determine if funds should be released from the vault.
    pub fn is_nft_in_collection_address(
        nft_mint: &Pubkey,
        collection_address: &Pubkey,
        token_account: &Account<TokenAccount>,
    ) -> bool {
        token_account.mint == *nft_mint && token_account.owner == *collection_address
    }

    /// Ensures that the token account matches the NFT mint and the expected owner.
    /// 
    /// If `owner` is None, it only checks if the mint matches, useful for when the NFT is in the collection address.
    pub fn validate_token_account(
        token_account: &Account<TokenAccount>,
        nft_mint: &Pubkey,
        owner: Option<&Pubkey>,
    ) -> Result<()> {
        if token_account.mint != *nft_mint {
            return Err(crate::errors::SuperVaultError::InvalidAccount.into());
        }
        if let Some(owner) = owner {
            if token_account.owner != *owner {
                return Err(crate::errors::SuperVaultError::InvalidAccount.into());
            }
        }
        Ok(())
    }
}