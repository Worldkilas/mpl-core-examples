use anchor_lang::prelude::*;
use mpl_core::instructions::TransferV1CpiBuilder;

use crate::SPL_NOOP_PROGRAM;

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: the mpl core program validates it so it is safe
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the NFT might belong to
    /// CHECK: THIS IS SAFE BECAUSE IT IS CHECKED BY THE MPL_CORE PROGRAM
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The owner or delegate who is authorized to perform lifecycle operations on the NFT
    /// e.g transferring and burning
    pub authority: Option<Signer<'info>>,

    /// The new owner of the NFT
    /// CHECK: This is safe because it is only a destination where the NFT will be sent
    pub new_owner: AccountInfo<'info>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    #[account(address=SPL_NOOP_PROGRAM)]
    pub log_wrapper: Option<AccountInfo<'info>>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is checked by the address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

impl<'info> TransferNFT<'info> {
    pub fn handler(&mut self) -> Result<()> {
        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(self.asset.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .new_owner(self.new_owner.to_account_info().as_ref())
            .system_program(Some(self.system_program.as_ref()))
            .authority(self.authority.as_deref())
            .log_wrapper(self.log_wrapper.as_ref())
            .invoke()?;
        Ok(())
    }
}
