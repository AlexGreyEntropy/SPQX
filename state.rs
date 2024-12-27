use anchor_lang::prelude::*;

/// Represents the Vault structure used for storing and managing SOL related to an NFT.
#[account]
pub struct Vault {
    /// The public key of the collection this NFT belongs to.
    pub collection: Pubkey,
    /// The mint address of the NFT.
    pub mint: Pubkey,
    /// The token account associated with the NFT.
    pub token_account: Pubkey,
    /// The current owner of the NFT, which is used to determine where funds should be sent when released.
    pub owner: Pubkey,
    /// The amount of SOL held in escrow for this vault, which is 20% of the royalty fees.
    pub escrow_balance: u64,
}

impl Vault {
    /// Constructs a new `Vault` instance with the provided details.
    ///
    /// # Arguments
    ///
    /// * `collection` - The public key of the collection the NFT is part of.
    /// * `mint` - The mint address of the NFT.
    /// * `token_account` - The token account where the NFT is held.
    /// * `owner` - The initial owner of the NFT.
    ///
    /// # Returns
    ///
    /// A new `Vault` instance.
    pub fn new(collection: Pubkey, mint: Pubkey, token_account: Pubkey, owner: Pubkey) -> Self {
        Vault {
            collection,
            mint,
            token_account,
            owner,
            escrow_balance: 0,
        }
    }

    /// Updates the owner of the vault. This should be called when the NFT is transferred to a new owner.
    ///
    /// # Arguments
    ///
    /// * `new_owner` - The new owner's public key.
    pub fn update_owner(&mut self, new_owner: Pubkey) {
        self.owner = new_owner;
    }

    /// Adds SOL to the vault's escrow balance. This function would typically be called 
    /// when a royalty fee is processed.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of SOL to add to the escrow balance.
    pub fn add_to_escrow(&mut self, amount: u64) {
        self.escrow_balance = self.escrow_balance.checked_add(amount).unwrap();
    }

    /// Releases all funds held in escrow back to the current owner. This should be 
    /// called when the NFT is returned to the collection address.
    ///
    /// # Returns
    ///
    /// The amount released from escrow.
    pub fn release_escrow(&mut self) -> u64 {
        let amount = self.escrow_balance;
        self.escrow_balance = 0;
        amount
    }
}