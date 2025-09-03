import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MplCoreExamples } from "../target/types/mpl_core_examples";
import {fetchAsset, mplCore} from "@metaplex-foundation/mpl-core";
import { Umi,  createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { assert } from "chai";

describe("mpl-core-examples", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider= anchor.getProvider();

  const connection= anchor.getProvider().connection;

  const program = anchor.workspace.mplCoreExamples as Program<MplCoreExamples>;

  let asset: anchor.web3.Keypair;

  let collection: anchor.web3.Keypair;

  let umi: Umi

  // set the config wallet as the payer of every tx
  const payer=anchor.getProvider().wallet;

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  // airdrop some SOL to the payer wallet and set up umi
  // we use UMI to fetch assets and see the results of our tests
  before(async()=>{
    connection.requestAirdrop(payer.publicKey,anchor.web3.LAMPORTS_PER_SOL*3).then(log);

    // umi=createUmi(connection);
    // // let umiKeypair=umi.eddsa.;
    // const umiSigner=createSignerFromKeypair(umi, anchor.getProvider().wallet );

    // umi.use(mplCore());
    // umi.use(signerIdentity(umiSigner));
  });

  // before each test, generate a new asset and collection
  beforeEach(async()=>{
    asset=anchor.web3.Keypair.generate();
    collection=anchor.web3.Keypair.generate();
  });

  describe("Collection creattion", ()=>{

  it("should create a collection", async () => {
    
    await program.methods.createCollection({
      name: "My NFT Collection",
      uri: "https://example.com/collection.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([payer.payer,collection]).rpc().then(log);
    console.log(`Collection: ${collection.publicKey.toBase58()}`);
});

  it("should create a collection with permanent transfer delegate plugin", async()=>{
    await program.methods.createCollectionWithPermanentTransferDelegate({
      name: "My NFT",
      uri: "https://example.com/collection.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
    }).signers([payer.payer, collection]).rpc().then(log);

    console.log(`Collection: ${collection.publicKey.toBase58()}`);
    
  })

  it("should create a collection with permanent freeze delegate plugin", async()=>{
    await program.methods.createCollectionWithPermanentFreezeDelegate({
      name: "My NFT",
      uri: "https://example.com/collection.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
    }).signers([payer.payer, collection]).rpc().then(log);

    console.log(`Collection: ${collection.publicKey.toBase58()}`);
    
  })

  it("should create a collection with permanent burn delegate plugin", async()=>{
    await program.methods.createCollectionWithPermanentFreezeDelegate({
      name: "My NFT",
      uri: "https://example.com/collection.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
    }).signers([payer.payer, collection]).rpc().then(log);

    console.log(`Collection: ${collection.publicKey.toBase58()}`);
    
  })

  it("should create a collection with permanent oracle plugin", async()=>{
    await program.methods.createCollectionWithOraclePlugin({
      name: "My NFT",
      uri: "https://example.com/collection.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
    }).signers([payer.payer, collection]).rpc().then(log);

    console.log(`Collection: ${collection.publicKey.toBase58()}`);
    
  })

  })

  describe("Master Edition flow",()=>{

  
  it("should create a collection with master edition and mint an edition as part of master edition", async()=>{
    const asset2= anchor.web3.Keypair.generate();
    const masterEdition=await program.methods.createMasterEdition({
      nameOfMasterEditionCollection: "My Master Edition Collection",
      uriOfMasterEditionCollection: "https://example.com/collection.json",
      maxSupply: 10,
      masterEditionName:"Test Master Edition",
      masterEditionUri:"https://example.com/edition.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
    }).signers([payer.payer, collection]).instruction();

    const edition=await program.methods.createEdition({
      nameOfEditionAsset: "My Edition NF1 #1",
      uriOfEditionAsset: "https://example.com/edition1.json",
    }).accountsPartial({
      payer: payer.publicKey,
      masterEditionCollection: collection.publicKey,
      authority: payer.publicKey,
      editionAsset: asset.publicKey,
      owner: payer.publicKey,
    }).signers([asset, payer.payer]).instruction();

    const edition2=await program.methods.createEdition({
      nameOfEditionAsset: "My Edition NF1 #2",
      uriOfEditionAsset: "https://example.com/edition1.json",
    }).accountsPartial({
      payer: payer.publicKey,
      masterEditionCollection: collection.publicKey, 
      authority: payer.publicKey,
      editionAsset: asset2.publicKey,
      owner: payer.publicKey,
    }).signers([asset2, payer.payer]).instruction();


    //bundle the 2 ixs into a tx
    const tx= new anchor.web3.Transaction().add(masterEdition, edition, edition2);

    await provider.sendAndConfirm(tx,[payer.payer, collection, asset,asset2]).then(log);
    console.log(`Asset: ${asset.publicKey.toBase58()}`);
    console.log(`Asset: ${asset.publicKey.toBase58()}`)
    console.log(`Collection: ${collection.publicKey.toBase58()}`); 
  })
  })

  describe("Covers the various asset creation flows",()=>{

  it("should create a standalone nft/asset", async()=>{
    await program.methods.createNft({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);
  })

  it("should create an asset as part of a collection", async()=>{
    // first create the collection
    await program.methods.createCollection({
      name: "My NFT Collection",
      uri: "https://example.com/collection.json",
    }).accountsPartial({
      payer: payer.publicKey,
      collection: collection.publicKey,
      updateAuthority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([payer.payer,collection]).rpc().then(log);

    console.log(`Collection: ${collection.publicKey.toBase58()}`);

    //then create asset as part of collection
    // set update authority to null since assets in a collection dont have
    // an update authority
    await program.methods.createNft({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    collection: collection.publicKey,
    authority: payer.publicKey,
    owner: payer.publicKey,
    updateAuthority:null
  }).signers([payer.payer, asset]).rpc().then(log);
    console.log(`Asset: ${asset.publicKey.toBase58()}`);
    
  })

  it("should create a nft/asset with permanent transfer delegate", async()=>{
    await program.methods.createNftWithPermanentTransferDelegate({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);

  })

  it("should create a nft/asset with permanent freeze delegate", async()=>{
    await program.methods.createNftWithPermanentFreezeDelegate({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);

  })

  it("should create a nft/asset with permanent burn delegate", async()=>{
    await program.methods.createNftWithPermanentBurnDelegate({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);

  })

  })

  describe("Covers various asset/nft operation like burn and transfer",()=>{

  it("should transfer asset after creation", async()=>{
    // first create the asset
    await program.methods.createNft({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);
  // then transfer asset to a wallet
  const destinationWallet= anchor.web3.Keypair.generate();

  await program.methods.transferNft().accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    collection:null,
    newOwner: destinationWallet.publicKey,
   
  }).signers([payer.payer]).rpc().then(log)
  })

  it("should burn asset after creation", async()=>{
    // first create the asset
    await program.methods.createNft({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);
  // then burn asset 
   await program.methods.burnNft().accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    collection:null,
    authority: payer.publicKey 
  }).signers([payer.payer]).rpc().then(log)
  })
  
  it("should create asset with metadata and then update metadata with another tx", async()=>{
    // first create the asset
    await program.methods.createNft({
      name: "My NFT",
      uri: "https://example.com/nft.json",
  }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    updateAuthority: payer.publicKey,
    authority: payer.publicKey,
    collection: null,
    owner: payer.publicKey,
  }).signers([payer.payer, asset]).rpc().then(log);

  // then update metadata
   await program.methods.updateNftMetadata({
    name: "Changed NFT name",
    uri: "https://eg.com/nft.json"
   }).accountsPartial({
    payer: payer.publicKey,
    asset: asset.publicKey,
    collection:null,
    updateAuthority: payer.publicKey 
  }).signers([payer.payer]).rpc().then(log)
  })
  })

  describe("Covers various cases of adding plugins at an asset level",()=>{
    it("should create asset and add royalties plugin",async()=>{
      await program.methods.createNft({
          name: "My NFT",
        uri: "https://example.com/nft.json"
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        updateAuthority: payer.publicKey,
        authority: payer.publicKey,
        collection: null,
        owner: payer.publicKey,
      }).signers([payer.payer, asset]).rpc().then(log)

      // Add royalties plugin
      await program.methods.addRoyaltiesPlugin({
        basisPoints: 600,
        creators:[
          {
            address: payer.publicKey,
            percentage: 100
          }
        ]
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        collection: null,
        authority:payer.publicKey,
      }).signers([payer.payer]).rpc().then(log)

    })


    it("should create asset and add autograph plugin",async()=>{
      await program.methods.createNft({
          name: "My NFT",
        uri: "https://example.com/nft.json"
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        updateAuthority: payer.publicKey,
        authority: payer.publicKey,
        collection: null,
        owner: payer.publicKey,
      }).signers([payer.payer, asset]).rpc().then(log)

      // Add autograph plugin
      await program.methods.addAutographPlugin({
        message: "Initialized message"
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        collection: null,
        authority:payer.publicKey,
      }).signers([payer.payer]).rpc().then(log)
    })

    it("should create asset and add royalty plugin, the update the royalties in another tx",async()=>{
      // 1. Create asset
      await program.methods.createNft({
          name: "My NFT",
        uri: "https://example.com/nft.json"
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        updateAuthority: payer.publicKey,
        authority: payer.publicKey,
        collection: null,
        owner: payer.publicKey,
      }).signers([payer.payer, asset]).rpc().then(log)

      // 2. Add royalties plugin
      const creator2= anchor.web3.Keypair.generate()
      await program.methods.addRoyaltiesPlugin({
        basisPoints: 500,
        creators:[
          {
            address: payer.publicKey,
            percentage: 100
          },
        
        ]
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        collection: null,
        authority:payer.publicKey,
      }).signers([payer.payer]).rpc().then(log)

      // 3. Update the creators list
      await program.methods.updateRoyaltiesPlugin({
        basisPoints: 700,
        creators:[
          {
            address: payer.publicKey,
            percentage: 50
          },
          {
            address: creator2.publicKey,
            percentage: 50
          }
        ],
      
      }).accountsPartial({
        payer: payer.publicKey,
        asset: asset.publicKey,
        collection: null,
        authority:payer.publicKey,
      }).signers([payer.payer]).rpc().then(log)
    })

    it("should create asset and add autograph plugin, then update the message in another tx",async()=>{
      // 1. Create asset
      await program.methods.createNft({
        name: "My NFT",
      uri: "https://example.com/nft.json"
    }).accountsPartial({
      payer: payer.publicKey,
      asset: asset.publicKey,
      updateAuthority: payer.publicKey,
      authority: payer.publicKey,
      collection: null,
      owner: payer.publicKey,
    }).signers([payer.payer, asset]).rpc().then(log)

    // 2. Add autograph plugin
    await program.methods.addAutographPlugin({ message: "Initialized message"}).accountsPartial({
      payer: payer.publicKey,
      asset: asset.publicKey,
      collection: null,
      authority:payer.publicKey,
    }).signers([payer.payer]).rpc().then(log)

    // 3. Update autograph plugin
    await program.methods.addNewAutographToAssetWithExistingAutographPlugin({ message: "Updated message"}).accountsPartial({
      payer: payer.publicKey,
      asset: asset.publicKey,
      collection: null,
      authority:payer.publicKey,
    }).signers([payer.payer]).rpc().then(log)

    })
  })

  describe("Some special use cases using mpl core",()=>{
    // One usecase is when you want to issue out soulbound NFTs
    // This can either be accomplished by creating an asset or collection with 
    // the permanent freeze delegate or an oracle deployed by metaplez that always 
    // rejects transfers. The difference between using any of the two is that with
    // permanent freeze delegate, the asset cannot be burnt.
    //! Note: A permanent plugin can only be created on asset creation
    // Here are some examples
    // Note that i'm doing most of these at collection level as it is more rent efficient

    it("should create a soulbound NFT with oracle at collection level. Then it should fail if transfer is attempted",async()=>{
      // 1. Create collection with permanent freeze delegate plugin
      await program.methods.createCollectionWithPermanentFreezeDelegate({
        name: "My collection",
        uri: "https://example.com/collection.json",
      }).accountsPartial({
        payer: payer.publicKey,
        collection: collection.publicKey,
        updateAuthority: payer.publicKey,
      }).signers([payer.payer, collection]).rpc().then(log)

      // 2. Mint asset as part of the collection
      await program.methods.createNft({
        name: "My NFT",
        uri: "https://example.com/nft.json",
      }).accountsPartial({
        payer:payer.publicKey,
        asset: asset.publicKey,
        collection: collection.publicKey,
        authority: payer.publicKey,
        updateAuthority: null,
        owner: payer.publicKey,
      }).signers([payer.payer, asset]).rpc().then(log)

      // 3. attempt to trasfer. test should fail if transfer successful
      try {
        // destinatination acccount
        const destinationWallet=anchor.web3.Keypair.generate()


        await program.methods.transferNft().accountsPartial({
          payer:payer.publicKey,
          asset:asset.publicKey,
          collection: collection.publicKey,
          authority:payer.publicKey,
          newOwner: destinationWallet.publicKey
        }).signers([payer.payer]).rpc().then(log)

        assert.fail("Transfer should have failed");
      } catch (err) {
        // const errMsg = (err as any).error.errorMessage;
        console.log(err);
     
      }
    });

    it("should create a soulbound NFT with metaplex oracle at collection level. Then it should fail if transfer is attempted",async()=>{
      // 1. Create asset with oracle plugin
      await program.methods.createCollectionWithOraclePlugin({
        name: "My NFT",
        uri: "https://example.com/nft.json",
      }).accountsPartial({
        payer: payer.publicKey,
    
        updateAuthority: payer.publicKey,

        collection: collection.publicKey,
    
      }).signers([payer.payer, collection]).rpc().then(log)

      // 2. Mint asset as part of the collection
    await program.methods.createNft({
      name: "My NFT",
      uri: "https://example.com/nft.json",
    }).accountsPartial({
      payer:payer.publicKey,
      asset: asset.publicKey,
      collection: collection.publicKey,
      authority: payer.publicKey,
      updateAuthority: null,
      owner: payer.publicKey,
    }).signers([payer.payer, asset]).rpc().then(log)

      
    // 3. attempt to trasfer. test should fail if transfer successful
    try {
      // destinatination acccount
      const destinationWallet=anchor.web3.Keypair.generate()

      await program.methods.transferNft().accountsPartial({
        payer:payer.publicKey,
        asset:asset.publicKey,
        collection: collection.publicKey,
        authority:payer.publicKey,
        newOwner: destinationWallet.publicKey
      }).signers([payer.payer]).rpc().then(log)

      assert.fail("Transfer should have failed");
    } catch (err) {
      // const errMsg = (err as any).error.errorMessage;
      console.log(err);

    }
      
    })

      it("Compressed NFTs using bubblegum", async()=>{
        await program.methods.addBubblegumPluginToCollection().accountsPartial({
          payer: payer.publicKey,
          collection: collection.publicKey,
          updateAuthority: payer.publicKey,
        });
      });
    });
  });


