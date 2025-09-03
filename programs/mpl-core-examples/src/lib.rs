pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;

declare_id!("5s9UNbsAjeJPfoSBdDNUZuGn89orurbmfDVJXuY4sW7d");

#[program]
pub mod mpl_core_examples {
    use super::*;

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts.create_collection(args)
    }

    pub fn create_collection_with_permanent_transfer_delegate(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts
            .create_collection_with_permanent_transfer_delegate(args)
    }

    pub fn create_collection_with_permanent_freeze_delegate(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts
            .create_collection_with_permanent_freeze_delegate(args)
    }

    pub fn create_collection_with_permanent_burn_delegate(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts
            .create_collection_with_permanent_burn_delegate(args)
    }

    pub fn create_collection_with_oracle_plugin(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts.create_collection_with_oracle_plugin(args)
    }

    pub fn create_master_edition(
        ctx: Context<CreateCollection>,
        args: CreateMasterEditionArgs,
    ) -> Result<()> {
        ctx.accounts.create_master_edition(args)
    }

    pub fn create_edition(ctx: Context<CreateEdition>, args: CreateEditionArgs) -> Result<()> {
        ctx.accounts.create_edition(args)
    }

    pub fn create_nft(ctx: Context<CreateNFT>, args: CreateNFTArgs) -> Result<()> {
        ctx.accounts.create_nft(args)
    }

    pub fn create_nft_with_permanent_transfer_delegate(
        ctx: Context<CreateNFT>,
        args: CreateNFTArgs,
    ) -> Result<()> {
        ctx.accounts
            .create_nft_with_permanent_transfer_delegate(args)
    }

    pub fn create_nft_with_permanent_freeze_delegate(
        ctx: Context<CreateNFT>,
        args: CreateNFTArgs,
    ) -> Result<()> {
        ctx.accounts.create_nft_with_permanent_freeze_delegate(args)
    }

    pub fn create_nft_with_permanent_burn_delegate(
        ctx: Context<CreateNFT>,
        args: CreateNFTArgs,
    ) -> Result<()> {
        ctx.accounts.create_nft_with_permanent_burn_delegate(args)
    }

    pub fn transfer_nft(ctx: Context<TransferNFT>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn burn_nft(ctx: Context<BurnNFT>) -> Result<()> {
        ctx.accounts.burn_nft()
    }

    pub fn update_nft_metadata(
        ctx: Context<UpdateNFTMetadata>,
        args: UpdateNFTMetadataArgs,
    ) -> Result<()> {
        ctx.accounts.handler(args)
    }


    pub fn add_royalties_plugin(
        ctx: Context<AddPluginsToNft>,
        args: AddRoyaltiesPluginArgs,
    ) -> Result<()> {
        ctx.accounts.add_royalties_plugin(args)
    }

    pub fn add_autograph_plugin(
        ctx: Context<AddPluginsToNft>,
        args: AddAutographPluginArgs,
    ) -> Result<()> {
        ctx.accounts.add_autograph_plugin_to_asset(args)
    }

    pub fn add_metaplex_oracle_to_collection(ctx: Context<AddPluginsToCollections>) -> Result<()> {
        ctx.accounts.add_metaplex_oracle_to_collection()
    }

    
    pub fn update_royalties_plugin(
        ctx: Context<UpdateNFTPlugin>,
        args: UpdateRoyaltiesPluginArgs,
    ) -> Result<()> {
        ctx.accounts.update_royalties_plugin(args)
    }

    pub fn update_freeze_plugin(ctx: Context<UpdateNFTPlugin>) -> Result<()> {
        ctx.accounts.update_freeze_delegate()
    }

    pub fn add_new_autograph_to_asset_with_existing_autograph_plugin(
        ctx: Context<UpdateNFTPlugin>,
        args:UpdateAutographPluginArgs
    ) -> Result<()> {
        ctx.accounts
            .add_new_autograph_to_asset_with_existing_autograph_plugin(args)
    }

    pub fn add_metaplex_oracle_to_nft(ctx: Context<AddPluginsToNft>)->Result<()> {
        ctx.accounts.add_metaplex_oracle_to_nft()
    }

    pub fn add_bubblegum_plugin_to_collection(ctx: Context<AddPluginsToCollections>)->Result<()> {
        ctx.accounts.add_bubblegum_plugin_to_collection()
    }
}
