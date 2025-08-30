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

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    /// Account paying for rent and creation of the collection
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The address of the new collection.
    #[account(mut)]
    pub collection: Signer<'info>,

    /// CHECK: Checked in mpl core
    pub update_authority: Option<AccountInfo<'info>>,

    /// The system program
    pub system_program: Program<'info, System>,

    /// The MPL core program
    /// CHECK: It is checked in the address constraint
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub uri: String,
}
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMasterEditionArgs {
    pub name_of_master_edition_collection: String,
    pub uri_of_master_edition_collection: String,
    pub master_edition_name: Option<String>,
    pub master_edition_uri: Option<String>,
    pub max_supply: u32,
}

impl<'info> CreateCollection<'info> {
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
