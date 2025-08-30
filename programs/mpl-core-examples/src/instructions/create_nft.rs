pub use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{CreateV1CpiBuilder, CreateV2CpiBuilder},
    types::{
        ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, OracleInitInfo,
        PermanentBurnDelegate, PermanentFreezeDelegate, PermanentTransferDelegate, Plugin,
        PluginAuthority, PluginAuthorityPair,
    },
};

use crate::ONCHAIN_METAPLEX_ORACLE_PLUGIN;

#[derive(Accounts)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub asset: Signer<'info>,

    /// The collection the asset will be part of.
    /// CHECK: Checked in mpl core
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The authority allowed to update the metadata of the asset. Default to authority if not present
    /// CHECK: Checked in mpl core
    pub update_authority: Option<AccountInfo<'info>>,

    /// Very important when creating an asset/NFT
    /// The authority authorizes the creation of an NFT and the tx fails if not present
    pub authority: Option<Signer<'info>>,

    /// The owner of the new asset. Defaults to the authority if not present.
    /// CHECK: Checked in mpl-core.
    pub owner: Option<AccountInfo<'info>>,

    pub system_program: Program<'info, System>,

    /// CHECK: It is checked in the address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateNFTArgs {
    pub name: String,
    pub uri: String,
}

impl<'info> CreateNFT<'info> {
    pub fn create_nft(&mut self, create_nft_args: CreateNFTArgs) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .asset(self.asset.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .authority(self.authority.as_deref())
            .owner(self.owner.as_ref())
            .update_authority(self.update_authority.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .name(create_nft_args.name)
            .uri(create_nft_args.uri)
            .invoke()?;
        Ok(())
    }

    pub fn create_nft_with_oracle_plugin(&mut self, args: CreateNFTArgs) -> Result<()> {
        CreateV2CpiBuilder::new(&self.mpl_core_program)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.to_account_info().as_ref())
            .name(args.name)
            .uri(args.uri)
            .authority(self.authority.as_deref())
            .owner(self.owner.as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .external_plugin_adapters(vec![ExternalPluginAdapterInitInfo::Oracle(
                OracleInitInfo {
                    base_address: ONCHAIN_METAPLEX_ORACLE_PLUGIN,
                    init_plugin_authority: None,
                    lifecycle_checks: vec![(
                        HookableLifecycleEvent::Transfer,
                        ExternalCheckResult { flags: 4 },
                    )],
                    base_address_config: None,
                    results_offset: Some(mpl_core::types::ValidationResultsOffset::Anchor),
                },
            )])
            .invoke()?;
        Ok(())
    }

    pub fn create_nft_with_permanent_freeze_delegate(&mut self, args: CreateNFTArgs) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.to_account_info().as_ref())
            .owner(self.owner.as_ref())
            .collection(self.collection.as_ref())
            .name(args.name)
            .system_program(self.system_program.to_account_info().as_ref())
            .uri(args.uri)
            .authority(self.authority.as_deref())
            .update_authority(self.update_authority.as_ref())
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: true }),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }

    pub fn create_nft_with_permanent_transfer_delegate(
        &mut self,
        args: CreateNFTArgs,
    ) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.to_account_info().as_ref())
            .owner(self.owner.as_ref())
            .collection(self.collection.as_ref())
            .name(args.name)
            .system_program(self.system_program.to_account_info().as_ref())
            .uri(args.uri)
            .authority(self.authority.as_deref())
            .update_authority(self.update_authority.as_ref())
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::PermanentTransferDelegate(PermanentTransferDelegate {}),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }

    pub fn create_nft_with_permanent_burn_delegate(&mut self, args: CreateNFTArgs) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.to_account_info().as_ref())
            .owner(self.owner.as_ref())
            .collection(self.collection.as_ref())
            .name(args.name)
            .system_program(self.system_program.to_account_info().as_ref())
            .uri(args.uri)
            .authority(self.authority.as_deref())
            .update_authority(self.update_authority.as_ref())
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;
        Ok(())
    }
}
