use anchor_lang::prelude::*;

#[error_code]
pub enum SuperVaultError {
    /// This error occurs when the NFT in the transaction is not associated with the specified collection.
    #[msg("NFT is not part of the collection.")]
    NotInCollection,

    /// Indicates that there are not enough funds in the account to execute the royalty distribution or other financial operations.
    #[msg("Insufficient funds for royalty distribution.")]
    InsufficientFunds,

    /// Triggered when trying to access or interact with a vault that does not exist or cannot be located in the program's state.
    #[msg("Vault account not found.")]
    VaultNotFound,

    /// This error is raised when the account passed to a function does not match the expected type, ownership, or other criteria.
    #[msg("Account provided does not match expected account.")]
    InvalidAccount,

    /// When the vault's owner does not match the current owner of the NFT, this error is thrown to prevent unauthorized access or updates.
    #[msg("The vault does not belong to the current NFT owner.")]
    VaultOwnerMismatch,

    /// Used when an operation is attempted on a vault that has been closed or is in an inactive state.
    #[msg("The vault is closed or inactive.")]
    VaultClosed,

    /// Thrown when an amount passed to a function is either negative or greater than a predefined maximum, ensuring data integrity.
    #[msg("Amount exceeds maximum allowed or is negative.")]
    InvalidAmount,

    /// Indicates that the NFT in question either has not been minted or the provided mint address is incorrect or invalid.
    #[msg("NFT has not been minted or mint address is invalid.")]
    NFTNotMinted,

    /// This error is for when an attempt to release funds from a vault is made, but the NFT isn't currently in the collection address.
    #[msg("NFT must be in the collection address to release funds.")]
    NFTNotInCollection,

    /// When there's an attempt to update the owner of an NFT, but the NFT involved in the transaction does not match the one expected.
    #[msg("Attempt to update owner for an NFT not in the transaction.")]
    UpdateOwnerMismatch,

    /// Indicates that the token account specified does not match the token account of the NFT that should be associated with the vault.
    #[msg("Token account does not match the NFT's token account.")]
    TokenAccountMismatch,

    /// Used when trying to create or interact with a vault for an NFT that already has an existing vault, preventing duplicate vaults.
    #[msg("Vault already exists for this NFT.")]
    VaultAlreadyExists,

    /// Thrown if an operation requires the NFT to be in circulation (not in the collection), but it isn't.
    #[msg("NFT is not in circulation.")]
    NFTNotCirculating,

    /// Error for when the vault signature verification fails, ensuring only authorized operations are performed.
    #[msg("Vault signature verification failed.")]
    VaultSignatureFailed,

    /// Used when there's an issue with the PDA (Program Derived Address) derivation, like incorrect seeds or program ID mismatch.
    #[msg("PDA derivation failed.")]
    PDADerivationFailed,

    /// This error indicates an attempt to interact with an NFT or vault state that's not in the expected state for the operation.
    #[msg("Incorrect state for operation.")]
    IncorrectState,

    /// For operations where time constraints are involved, this error indicates that the time condition wasn't met.
    #[msg("Time condition not satisfied.")]
    TimeConditionFailed,
}