use anchor_lang::prelude::*;
use mpl_core::{
    fetch_asset_plugin,
    instructions::CreateV1CpiBuilder,
    types::{Edition, Plugin, PluginAuthority, PluginAuthorityPair, PluginType},
};

use crate::state::EditionCouter;

#[derive(Accounts)]
pub struct CreateEdition<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Th
    #[account(mut)]
    pub edition_asset: Signer<'info>,

    /// The collection the asset will be part of.
    /// CHECK: Checked in mpl core
    #[account(mut)]
    pub master_edition_collection: AccountInfo<'info>,

    /// Very important when creating an asset/NFT
    /// The authority authorizes the creation of an NFT and the tx fails if not present
    pub authority: Option<Signer<'info>>,

    /// The owner of the new asset. Defaults to the authority if not present.
    /// CHECK: Checked in mpl-core.
    pub owner: Option<AccountInfo<'info>>,

    #[account(
        init_if_needed,
        payer=payer,
        space=8+EditionCouter::INIT_SPACE,
        seeds=[b"edition_counter", master_edition_collection.key().as_ref()],
        bump
    )]
    pub edition_count: Account<'info, EditionCouter>,

    pub system_program: Program<'info, System>,

    /// CHECK: It is checked in the address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateEditionArgs {
    pub name_of_edition_asset: String,
    pub uri_of_edition_asset: String,
}

impl<'info> CreateEdition<'info> {
    pub fn create_edition(&mut self, create_edition_args: CreateEditionArgs) -> Result<()> {
        let counter = &mut self.edition_count;
        counter.edition_count += 1;

        let edition_number = counter.edition_count;

        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .collection(Some(self.master_edition_collection.as_ref()))
            .asset(self.edition_asset.as_ref())
          
            .system_program(self.system_program.to_account_info().as_ref())
            .owner(self.owner.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(create_edition_args.uri_of_edition_asset)
            .name(create_edition_args.name_of_edition_asset)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Edition(Edition {
                    number: edition_number,
                }),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;

        Ok(())
    }
}
