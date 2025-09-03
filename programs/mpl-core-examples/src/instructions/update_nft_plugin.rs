use anchor_lang::prelude::*;
use mpl_core::{
    fetch_asset_plugin,
    instructions::UpdatePluginV1CpiBuilder,
    types::{
        AutographSignature, Creator, FreezeDelegate, Plugin, PluginType, Royalties,
    },
};

use crate::{CreatorArgs, SPL_NOOP_PROGRAM};

/// Updating an existing asset/NFT can be done on the client side using UMI
/// But here I take some examples of instances where you can update plugins from an anchor program
/// The same can be done for collections as well, it's just a matter of passing the collection address instead of the asset address
/// and then chainging the CPI name from `UpdatePluginV1CpiBuilder` to `UpdateCollectionPluginV1CpiBuilder`
#[derive(Accounts)]
pub struct UpdateNFTPlugin<'info> {
    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the asset might belong to.
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
    #[account(address=SPL_NOOP_PROGRAM)]
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateRoyaltiesPluginArgs {
    pub basis_points: u16,
    pub creators: Vec<CreatorArgs>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateAutographPluginArgs {
    pub message:String
}

impl<'info> UpdateNFTPlugin<'info> {
    /// Example of updating the royalties plugin of an existing NFT
    pub fn update_royalties_plugin(&mut self, args: UpdateRoyaltiesPluginArgs) -> Result<()> {
        // Map the incoming creators into the expected `Creator` format.
        let creators = args
            .creators
            .into_iter()
            .map(|c| Creator {
                address: c.address,
                percentage: c.percentage,
            })
            .collect();

        UpdatePluginV1CpiBuilder::new(&self.mpl_core)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .authority(self.authority.as_deref())
            .system_program(self.system_program.to_account_info().as_ref())
            .log_wrapper(self.log_wrapper.as_ref())
            .plugin(Plugin::Royalties(Royalties {
                basis_points: args.basis_points,
                creators,
                rule_set: mpl_core::types::RuleSet::None,
            }))
            .invoke()?;
        Ok(())
    }

    /// Let's try an example of updating a state based plugin like freeze delegate where frozen can be set to true or false
    pub fn update_freeze_delegate(&mut self) -> Result<()> {
        // First we need to fetch the existing plugin data to know the current state
        let (_, freeze_delegate, __) =
            fetch_asset_plugin::<FreezeDelegate>(self.asset.as_ref(), PluginType::FreezeDelegate)?;

        UpdatePluginV1CpiBuilder::new(&self.mpl_core)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .authority(self.authority.as_deref())
            .system_program(self.system_program.to_account_info().as_ref())
            .log_wrapper(self.log_wrapper.as_ref())
            // Here we toggle the frozen state
            .plugin(Plugin::FreezeDelegate(FreezeDelegate {
                frozen: !freeze_delegate.frozen,
            }))
            .invoke()?;

        Ok(())
    }

    /// For working with plugins with more complex data like autographs and verified creators,
    /// you'll need to pass the complete list you want to maintain when updating these plugins
    /// since it maintains the full list of data.
    ///
    /// See example below
    pub fn add_new_autograph_to_asset_with_existing_autograph_plugin(&mut self, args: UpdateAutographPluginArgs) -> Result<()> {
        // Fetch the existing autograph plugin data
        let (_, mut existing_autograph_plugin, __) = fetch_asset_plugin::<
            mpl_core::types::Autograph,
        >(
            self.asset.as_ref(), PluginType::Autograph
        )?;
        // Add a new signature to the existing list and return it as the updated plugin data
        let updated_autograph_plugin = {
            existing_autograph_plugin
                .signatures
                .push(AutographSignature {
                    address: self.payer.key(),
                    message: args.message,
                });
            existing_autograph_plugin
        };

        // Pass updated plugin data to the update instruction
        UpdatePluginV1CpiBuilder::new(&self.mpl_core)
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.as_ref())
            .collection(self.collection.as_ref())
            .authority(self.authority.as_deref())
            .system_program(self.system_program.to_account_info().as_ref())
            .log_wrapper(self.log_wrapper.as_ref())
            .plugin(Plugin::Autograph(updated_autograph_plugin));
        Ok(())
    }
}
