use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{CreateCollectionV1CpiBuilder, CreateCollectionV2CpiBuilder},
    types::{
        ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, MasterEdition,
        OracleInitInfo, PermanentBurnDelegate, PermanentFreezeDelegate, PermanentTransferDelegate,
        Plugin, PluginAuthority, PluginAuthorityPair, ValidationResultsOffset,
    },
};

use crate::{MPL_CORE_ID, ONCHAIN_METAPLEX_ORACLE_PLUGIN};

/// Accounts required for creating a new collection or a collection with plugins.
#[derive(Accounts)]
pub struct CreateCollection<'info> {
    /// Account paying for rent and transaction fees.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The new collection account to be created.
    #[account(mut)]
    pub collection: Signer<'info>,

    /// Optional update authority for the collection.
    /// CHECK: Validated by the MPL Core program.
    pub update_authority: Option<AccountInfo<'info>>,

    /// Solana System Program (for account creation).
    pub system_program: Program<'info, System>,

    /// Metaplex Core program (CPI target).
    /// CHECK: Address constraint ensures correctness.
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

/// Arguments for creating a standard collection.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionArgs {
    /// Human-readable name of the collection.
    pub name: String,
    /// Metadata URI describing the collection.
    pub uri: String,
}

/// Arguments for creating a master edition collection.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMasterEditionArgs {
    /// Name of the master edition collection.
    pub name_of_master_edition_collection: String,
    /// Metadata URI of the master edition collection.
    pub uri_of_master_edition_collection: String,
    /// Optional name for the Master Edition plugin.
    pub master_edition_name: Option<String>,
    /// Optional URI for the Master Edition plugin.
    pub master_edition_uri: Option<String>,
    /// Maximum number of editions allowed.
    pub max_supply: u32,
}

impl<'info> CreateCollection<'info> {
    /// Creates a basic collection with a name and URI.
    pub fn create_collection(&mut self, args: CreateCollectionArgs) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri)
            .name(args.name)
            .invoke()?;
        Ok(())
    }

    /// Creates a master edition collection with a `MasterEdition` plugin.
    pub fn create_master_edition(&mut self, args: CreateMasterEditionArgs) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri_of_master_edition_collection)
            .name(args.name_of_master_edition_collection)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::MasterEdition(MasterEdition {
                    max_supply: Some(args.max_supply),
                    name: args.master_edition_name,
                    uri: args.master_edition_uri,
                }),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }

    /// Creates a collection with a permanent transfer delegate plugin.
    pub fn create_collection_with_permanent_transfer_delegate(
        &mut self,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri)
            .name(args.name)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::PermanentTransferDelegate(PermanentTransferDelegate {}),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }

    /// Creates a collection with a permanent freeze delegate plugin.
    /// By default, the collection is initialized in a frozen state (`frozen = true`).
    pub fn create_collection_with_permanent_freeze_delegate(
        &mut self,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri)
            .name(args.name)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: true }),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }

    /// Creates a collection with a permanent burn delegate plugin.
    pub fn create_collection_with_permanent_burn_delegate(
        &mut self,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri)
            .name(args.name)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }

    /// Creates a collection with an external oracle plugin.
    /// This plugin allows external validation checks (e.g., transfer rules).
    pub fn create_collection_with_oracle_plugin(
        &mut self,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri)
            .name(args.name)
            .external_plugin_adapters(vec![ExternalPluginAdapterInitInfo::Oracle(
                OracleInitInfo {
                    base_address: ONCHAIN_METAPLEX_ORACLE_PLUGIN,
                    init_plugin_authority: None,
                    lifecycle_checks: vec![(
                        HookableLifecycleEvent::Transfer,
                        ExternalCheckResult { flags: 4 },
                    )],
                    base_address_config: None,
                    results_offset: Some(ValidationResultsOffset::Anchor),
                },
            )])
            .invoke()?;
        Ok(())
    }
}
