use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token};

declare_id!("GLNFT");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, _bump: u8) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        vault.mint = *ctx.accounts.mint.key;
        vault.token_id = *ctx.accounts.token_id.key;
        vault.collection = *ctx.accounts.collection.key;
        vault.escrow_wallet = *ctx.accounts.escrow_wallet.key;
        vault.amount_sol = (*ctx.accounts.mint.amount_sol / 2) - *ctx.accounts.royalty_fee.amount_sol;
        // SOL amount in vault allocated is 50% of the total mint amount, after royalty fees
        vault.amount_token = (*ctx.accounts.mint.amount_token / 2) - *ctx.accounts.royalty_fee.amount_token;
        // Token amount in vault allocated is 50% of the total mint amount in Tokens, after royalty fees
        
        Ok(())
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        // Logic to update metadata
        // Assuming metadata fields are SOL and Token
        vault.SOL = ctx.accounts.vault.amount_sol;
        vault.Token = ctx.accounts.vault.amount_token;

        Ok(())
    }

    pub fn unlock_vault(ctx: Context<UnlockVault>) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;

        // Check if the programmable NFT is sent to the collection address
        if *ctx.accounts.pnft.owner != *ctx.accounts.collection.key {
            return Err(ProgramError::InvalidArgument);
        }

        // Unlock SOL amount to the owner
        **ctx.accounts.owner_wallet.to_account_info().try_borrow_mut_lamports()? += vault.amount_sol;
        **ctx.accounts.escrow_wallet.to_account_info().try_borrow_mut_lamports()? -= vault.amount_sol;

        // Unlock Token amount to the owner
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.escrow_wallet.to_account_info(),
            to: ctx.accounts.owner_wallet.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, vault.amount_token)?;

        // Change owner to the on-chain collection address
        vault.owner = *ctx.accounts.collection.key;

        // Delete the vault
        ctx.accounts.vault.to_account_info().close(ctx.accounts.owner.to_account_info())?;

        Ok(())
    }
        // Logic to handle transaction, 420 basis points of the transaction amount is the total retained as royalty fee
    pub fn handle_transaction(ctx: Context<HandleTransaction>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        let royalty_fee = amount_sol;
        let sol_fee = royalty_fee / 10;
        let token_fee = royalty_fee / 10;

        // Retain 42 points in SOL
        vault.amount_sol += sol_fee;
        **ctx.accounts.escrow_wallet.to_account_info().try_borrow_mut_lamports()? += sol_fee;

        // Retain 42 points in Token
        vault.amount_token += token_fee;
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.owner_wallet.to_account_info(),
            to: ctx.accounts.escrow_wallet.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, token_fee)?;

        Ok(())
    }
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub token_id: Pubkey,
    pub collection: Pubkey,
    pub escrow_wallet: Pubkey,
    pub amount_sol: u64,
    pub amount_token: u64,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = owner, space = 8 + 256, seeds = [mint.key().as_ref(), token_id.key().as_ref(), collection.key().as_ref(), owner.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    pub mint: AccountInfo<'info>,
    pub token_id: AccountInfo<'info>,
    pub collection: AccountInfo<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub escrow_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}

#[derive(Accounts)]
pub struct UnlockVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub escrow_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pnft: Account<'info, TokenAccount>,
    pub collection: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
} 

#[derive(Accounts)]
pub struct HandleTransaction<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}