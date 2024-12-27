use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseCollectionV1},
    instructions::{AddPluginV1CpiBuilder},
    types::{Plugin, UpdateAuthority},
};

/// Context for attaching the plugin to a collection
#[derive(Accounts)]
pub struct AttachPluginToCollection<'info> {
    /// The collection to attach the plugin to
    #[account(mut)]
    pub collection: Account<'info, BaseCollectionV1>,
    /// The authority that can update the collection
    pub update_authority: Signer<'info>,
    /// The Metaplex program to execute the CPI call
    pub metaplex_program: Program<'info, mpl_core::program::MplCore>,
}

/// Adds the plugin to a collection
pub fn attach_plugin_to_collection(
    ctx: Context<AttachPluginToCollection>,
    plugin_address: Pubkey,
) -> Result<()> {
    let cpi_accounts = AddPluginV1CpiBuilder::new(&ctx.accounts.collection)
        .with_update_authority(&ctx.accounts.update_authority);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.metaplex_program.to_account_info(),
        cpi_accounts,
    );

    // Invoke the CPI to add the plugin to the collection
    AddPluginV1CpiBuilder::new(cpi_ctx)
        .with_plugin(Plugin { address: plugin_address })
        .invoke()?;

    Ok(())
}