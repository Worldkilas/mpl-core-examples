use anchor_lang::prelude::*;
use mpl_core::instructions::BurnV1CpiBuilder;

use crate::MPL_CORE_ID;

#[derive(Accounts)]
pub struct BurnNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: The Nft asset to be burned
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// CHECK: The collection the asset/NFT might belong to
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    pub authority: Option<Signer<'info>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in the address constraint
    #[account(address=MPL_CORE_ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

impl<'info> BurnNFT<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {
        BurnV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .payer(self.payer.to_account_info().as_ref())
            .asset(self.asset.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .authority(self.authority.as_deref())
            .system_program(Some(self.system_program.to_account_info().as_ref()))
            .invoke()?;
        Ok(())
    }
}
