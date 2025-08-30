use anchor_lang::prelude::*;
use mpl_core::{
    instructions::AddPluginV1CpiBuilder,
    types::{
        Autograph, AutographSignature, BurnDelegate, Creator, FreezeDelegate, Plugin, Royalties,
        TransferDelegate,
    },
};

use crate::MPL_CORE_ID;

#[derive(Accounts)]
pub struct AddPluginsToNft<'info> {
    /// The address of the asset that will receive the plugins.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the asset might belong to. If it the asset/nft is part of a collection,
    /// the collection must have been created with the `CreateCollectionV1` instruction.
    /// The instruction will fail if the asset is part of a collection and the collection is not provided.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The account paying for the storage fees.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,

    /// The system program.
    pub system_program: Program<'info, System>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = MPL_CORE_ID)]
    pub mpl_core: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreatorArgs {
    pub address: Pubkey,
    pub percentage: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddRoyaltiesPluginArgs {
    pub basis_points: u16,
    pub creators: Vec<CreatorArgs>,
}



impl<'info> AddPluginsToNft<'info> {
    pub fn add_plugins() {
        
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
    pub fn add_royalties_plugin(&mut self, args: AddRoyaltiesPluginArgs) -> Result<()> {
        // Map the incoming creators into the expected `Creator` format.
        let creators = args
            .creators
            .into_iter()
            .map(|c| Creator {
                address: c.address,
                percentage: c.percentage,
            })
            .collect();

        // Build and invoke the CPI to add the royalties plugin.
        AddPluginV1CpiBuilder::new(&self.mpl_core)
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .authority(self.authority.as_deref())
            .plugin(Plugin::Royalties(Royalties {
                basis_points: args.basis_points,
                creators,
                rule_set: mpl_core::types::RuleSet::None,
            }))
            .invoke()?;
        Ok(())
    }

    /// Adds an "Autograph" plugin to the asset.
    /// This plugin allows recording of digital signatures/messages
    /// from authorities (e.g. creator autographing their NFT).
    pub fn add_autograph_plugin_to_asset(&mut self) -> Result<()> {
        AddPluginV1CpiBuilder::new(&self.mpl_core)
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .authority(self.authority.as_deref())
            .plugin(Plugin::Autograph(Autograph {
                signatures: vec![AutographSignature {
                    message: "Example messages".to_string(),
                    address: self
                        .authority
                        .as_ref()
                        .map(|auth| auth.key())
                        .unwrap_or_default(),
                }],
            }))
            .invoke()?;
        Ok(())
    }

    /// Adds a Transfer Delegate plugin.
    ///  An **Owner-Managed** plugin that allows a designated delegate to transfer
    /// the asset on behalf of the owner. Useful for cases such as:
    /// - Escrowless sales: transfer NFTs directly to buyers without escrow.
    /// - Gaming: automatically move assets when in-game events occur.
    /// - Subscriptions: transfer NFTs as part of recurring service logic.
    ///
    /// ⚠️ Warning:
    /// The transfer delegate authority is **temporary**. It is cleared
    /// whenever the asset is transferred, meaning delegates do not persist
    /// across owners. This prevents infinite delegation chains and ensures
    /// clean ownership semantics.
    pub fn add_transfer_delegate_plugin_to_asset(&mut self) -> Result<()> {
        AddPluginV1CpiBuilder::new(&self.mpl_core)
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .authority(self.authority.as_deref())
            .plugin(Plugin::TransferDelegate(TransferDelegate {}))
            .invoke()?;
        Ok(())
    }

    /// Freeze Plugin
    ///
    /// An **Owner-Managed** plugin that allows the asset to be *frozen*,
    /// preventing any transfers while active. The plugin authority may
    /// unfreeze the asset or revoke their authority at any time.
    ///
    /// Common use cases:
    /// - Escrowless staking: freeze NFTs while staked without transferring to escrow.
    /// - Escrowless marketplace listings: list NFTs without moving them to marketplace custody.
    /// - Gaming: temporarily lock in-game items during active gameplay.
    /// - Rentals: freeze NFTs while rented out to ensure non-transferability.
    /// - Governance: lock governance tokens during voting or proposal participation.
    /// - Collateral: lock NFTs being used in lending protocols as security.
    /// - Tournaments: lock NFTs while they are used in competitions.
    ///
    /// ⚠️ Warning:
    /// A frozen asset **cannot be transferred** until unfrozen. This ensures
    /// strong guarantees for protocols that require immovable assets, but
    /// requires the plugin authority to explicitly unfreeze before transfers
    /// are possible.
    pub fn add_freeze_delegate_plugin_to_asset(&mut self) -> Result<()> {
        AddPluginV1CpiBuilder::new(&self.mpl_core)
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .authority(self.authority.as_deref())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: true }))
            .invoke()?;
        Ok(())
    }

    /// Burn Plugin
    ///
    /// An **Owner-Managed** plugin that grants the plugin authority the power
    /// to permanently burn (destroy) the asset at any time.
    ///
    /// Common use cases:
    /// - Gaming: automatically burn NFTs when certain in-game events occur
    ///   (e.g., item durability breaks, character perma-death).
    /// - Limited collectibles: enforce scarcity by allowing protocols to
    ///   programmatically burn assets.
    /// - Redemption flows: burn an NFT when it is redeemed for a reward or
    ///   physical item.
    /// - Compliance: remove assets in scenarios where regulations or policies
    ///   require destruction.
    ///
    /// ⚠️ Warning:
    /// Burn is **irreversible** — once executed, the asset is destroyed and
    /// cannot be recovered.
    pub fn add_burn_delegate_plugin_to_asset(&mut self) -> Result<()> {
        AddPluginV1CpiBuilder::new(&self.mpl_core)
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .authority(self.authority.as_deref())
            .plugin(Plugin::BurnDelegate(BurnDelegate {}))
            .invoke()?;
        Ok(())
    }
}
