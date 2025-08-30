use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateV1CpiBuilder,
    types::{Edition, Plugin, PluginAuthority, PluginAuthorityPair},
};

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
pub struct CreateEditionArgs {
    pub name_of_edition_asset: String,
    pub uri_of_edition_asset: String,
}

impl<'info> CreateEdition<'info> {
    pub fn create_edition(&mut self, create_edition_args: CreateEditionArgs) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .collection(Some(self.master_edition_collection.as_ref()))
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .owner(self.owner.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(create_edition_args.uri_of_edition_asset)
            .name(create_edition_args.name_of_edition_asset)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Edition(Edition { number: 1 }),
                authority: Some(PluginAuthority::UpdateAuthority),
            }])
            .invoke()?;

        Ok(())
    }
}
