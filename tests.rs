use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token;
use super_vault_plugin::{
    self,
    state::{Vault},
    errors::SuperVaultError,
};
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::Transaction};

mod program_test {
    use super::*;

    #[tokio::test]
    async fn test_initialize_vault() {
        // Setup context
        let program_id = super_vault_plugin::id();
        let mut program_test = ProgramTest::new(
            "super_vault_plugin",
            program_id,
            processor!(super_vault_plugin::process_instruction),
        );
        
        // Add necessary accounts
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Mock accounts
        let collection = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let (vault_pda, _) = Pubkey::find_program_address(
            &[b"vault", collection.as_ref(), mint.as_ref(), token_account.as_ref(), payer.pubkey().as_ref()],
            &program_id,
        );

        // Create accounts for testing
        let rent = banks_client.get_rent().await.unwrap();
        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &payer.pubkey(),
                &vault_pda,
                rent.minimum_balance(Vault::LEN),
                Vault::LEN as u64,
                &program_id,
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Test initialization
        let ix = super_vault_plugin::instruction::InitializeVault {
            vault: vault_pda,
            collection: collection,
            asset: mint,
            payer: payer.pubkey(),
            amount: 1_000_000,
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix.into()],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await.unwrap();

        // Verify vault state
        let vault_account = banks_client.get_account(vault_pda).await.unwrap().unwrap();
        let vault = Vault::try_deserialize(&mut vault_account.data.as_slice()).unwrap();
        assert_eq!(vault.collection, collection);
        assert_eq!(vault.mint, mint);
        assert_eq!(vault.token_account, token_account);
        assert_eq!(vault.owner, payer.pubkey());
        assert_eq!(vault.escrow_balance, 0); // Initial balance should be 0
    }

    #[tokio::test]
    async fn test_update_vault_owner() {
        // ... Similar setup as above ...

        // Create a new owner for testing
        let new_owner = Keypair::new();
        
        // ... Process transaction to update owner ...

        // Verify the new owner
        let vault_account = banks_client.get_account(vault_pda).await.unwrap().unwrap();
        let vault = Vault::try_deserialize(&mut vault_account.data.as_slice()).unwrap();
        assert_eq!(vault.owner, new_owner.pubkey());
    }

    #[tokio::test]
    async fn test_process_royalty() {
        // ... Setup similar to above ...

        // Process royalty with some amount
        let royalty_amount = 100_000;
        
        // ... Process royalty transaction ...

        // Check if 20% of royalty went to vault and 80% to creator
        let vault_account = banks_client.get_account(vault_pda).await.unwrap().unwrap();
        let vault = Vault::try_deserialize(&mut vault_account.data.as_slice()).unwrap();
        assert_eq!(vault.escrow_balance, royalty_amount * 20 / 100);
        
        // Check creator received 80%
        let creator_balance = banks_client.get_balance(creator.pubkey()).await.unwrap();
        assert_eq!(creator_balance, royalty_amount * 80 / 100);
    }

    #[tokio::test]
    async fn test_release_funds() {
        // ... Setup with funds in the vault ...

        // Simulate returning NFT to collection
        // ... Process release funds transaction ...

        // Verify funds were released correctly
        let vault_account = banks_client.get_account(vault_pda).await.unwrap().unwrap();
        let vault = Vault::try_deserialize(&mut vault_account.data.as_slice()).unwrap();
        assert_eq!(vault.escrow_balance, 0);

        let last_owner_balance = banks_client.get_balance(last_owner.pubkey()).await.unwrap();
        // Ensure last_owner received the funds
        assert!(last_owner_balance > 0); // Exact amount depends on previous transactions
    }

    #[tokio::test]
    async fn test_error_conditions() {
        // ... Setup for testing error conditions ...

        // Test for NotInCollection error
        // ... Attempt to initialize vault with an NFT not in the collection ...

        // Test for InsufficientFunds
        // ... Attempt to process royalty with insufficient funds ...

        // Add more error condition tests as needed
    }
}