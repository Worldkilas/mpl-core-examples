use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{
        AddCollectionExternalPluginAdapterV1CpiBuilder, AddCollectionPluginV1CpiBuilder,
    },
    types::{
        BubblegumV2, Creator, ExternalCheckResult, ExternalPluginAdapterInitInfo,
        HookableLifecycleEvent, OracleInitInfo, Royalties,
    },
};

use crate::{AddRoyaltiesPluginArgs, ONCHAIN_METAPLEX_ORACLE_PLUGIN};

/// For some plugins. it better to add them at the collection-level because it is more rent efficient
/// I'll take some examples here

#[derive(Accounts)]
pub struct AddPluginsToCollections<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The address of the collection.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: AccountInfo<'info>,

    /// The owner or delegate of the asset.
    pub update_authority: Option<Signer<'info>>,

    /// The system program.
    pub system_program: Program<'info, System>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

impl<'info> AddPluginsToCollections<'info> {
    /// Adds the **Bubblegum Plugin** to a Collection.
    ///
    /// The **Bubblegum Plugin** is an *Authority-Managed* plugin that enables
    /// a Collection to be used as the basis for **compressed NFTs**.
    ///
    /// Notes:
    /// - This plugin can **only** be applied to **MPL Core Collections**.
    /// - It is not supported for individual **MPL Core Assets**.
    ///
    /// Common use cases:
    /// - Large-scale minting of NFTs where compression is required.
    /// - Gaming or social apps that need lightweight NFT storage at scale.
    /// - Any scenario where collection-level compression enables massive
    ///   asset issuance without high on-chain storage costs.
    pub fn add_bubblegum_plugin_to_collection(&mut self) -> Result<()> {
        AddCollectionPluginV1CpiBuilder::new(&self.mpl_core)
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .authority(self.update_authority.as_deref())
            .log_wrapper(self.log_wrapper.as_ref())
            .plugin(mpl_core::types::Plugin::BubblegumV2(BubblegumV2 {}))
            .invoke()?;
        Ok(())
    }

    /// Adds a royalties plugin to an asset or collection.
    ///
    /// This function configures royalty distribution for an asset or collection by attaching  
    /// a `Royalties` plugin through an MPL Core CPI call. Royalties specify a percentage fee  
    /// (basis points) and a list of creators who will receive payouts whenever the asset  
    /// is sold or used in a royalty-enforcing context.
    ///
    /// Note: This can also be added at collection-level
    ///
    /// # Arguments
    ///
    /// * `args` - An [`AddRoyaltiesPluginArgs`] struct containing:
    ///   - `basis_points`: The royalty percentage in basis points (1% = 100 bps).  
    ///   - `creators`: A vector of creator structs, each specifying an address and percentage share.
    pub fn add_royalties_plugin_to_collection(
        &mut self,
        args: AddRoyaltiesPluginArgs,
    ) -> Result<()> {
        // Map the incoming creators into the expected `Creator` format.
        let creators = args
            .creators
            .into_iter()
            .map(|c| Creator {
                address: c.address,
                percentage: c.percentage,
            })
            .collect();
        AddCollectionPluginV1CpiBuilder::new(&self.mpl_core)
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .authority(self.update_authority.as_deref())
            .log_wrapper(self.log_wrapper.as_ref())
            .plugin(mpl_core::types::Plugin::Royalties(Royalties {
                creators,
                basis_points: args.basis_points,
                rule_set: mpl_core::types::RuleSet::None,
            }))
            .invoke()?;
        Ok(())
    }

    pub fn add_metaplex_oracle_to_collection(&mut self) -> Result<()> {
        AddCollectionExternalPluginAdapterV1CpiBuilder::new(&self.mpl_core)
            .payer(self.payer.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .init_info(ExternalPluginAdapterInitInfo::Oracle(OracleInitInfo {
                base_address: ONCHAIN_METAPLEX_ORACLE_PLUGIN,
                init_plugin_authority: None,
                lifecycle_checks: vec![(
                    HookableLifecycleEvent::Transfer,
                    ExternalCheckResult { flags: 4 },
                )],
                base_address_config: None,
                results_offset: Some(mpl_core::types::ValidationResultsOffset::Anchor),
            }))
            .invoke()?;
        Ok(())
    }
}
