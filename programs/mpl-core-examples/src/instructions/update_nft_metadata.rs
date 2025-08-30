use anchor_lang::prelude::*;
use mpl_core::instructions::UpdateV1CpiBuilder;

use crate::MPL_CORE_ID;

#[derive(Accounts)]
pub struct UpdateNFTMetadata<'info> {
    /// The one who pays for the transaction(i.e updating the metadata)
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The asset/NFT whose metadata is being updated
    /// CHECK: The mpl core program handles the check
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection the asset might be part of.
    /// CHECK: Checked in mpl core
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The authority allowed to update the metadata of the asset.
    /// CHECK: Checked in mpl core
    pub update_authority: Option<AccountInfo<'info>>,

    /// CHECK: The address constraint is used to ensure this is the mpl core program
    #[account(address=MPL_CORE_ID)]
    pub mpl_core_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateNFTMetadataArgs {
    pub name: String,
    pub uri: String,
}

impl<'info> UpdateNFTMetadata<'info> {
    pub fn handler(&mut self, args: UpdateNFTMetadataArgs) -> Result<()> {
        UpdateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(self.asset.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .new_name(args.name)
            .new_uri(args.uri)
            .authority(self.update_authority.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .invoke()?;
        Ok(())
    }
}
