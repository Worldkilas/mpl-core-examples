# 🪄 MPL Core Examples

A complete Anchor program showcasing how to use **Metaplex Core** on Solana.  
This repo focuses on **real-world workflows** with **Assets**, **Collections**, and **Plugins**, covering creation, customization, and lifecycle operations.  

---

## 📚 Table of Contents

- [What is Metaplex Core?](#-what-is-metaplex-core)
- [Core Concepts](#-core-concepts)
  - [Assets](#assets)
  - [Collections](#collections)
  - [Plugins & Lifecycle](#plugins--lifecycle)
- [Program Features](#-program-features)
- [Special Demonstrations](#-special-demonstrations)
- [Prerequisites](#-prerequisites)
- [Getting Started](#-getting-started)
- [Example Usage](#-example-usage)
- [Tests](#-tests)
- [FAQs](#-faqs)
- [Resources](#-resources)

---

## 🌟 What is Metaplex Core?

[Metaplex Core](https://developers.metaplex.com/core) is the next-generation NFT standard on Solana.  
It introduces:
- A **single-account design** → cheaper mints (≈0.0037 SOL vs. 0.0220 SOL with Token Metadata).  
- A **plugin system** → extend Assets/Collections with royalties, transfer rules, autographs, and more.  

This repo demonstrates Core in action:
- Creating **Collections**, **Master Editions**, **Editions**, and **NFTs**.  
- Using **plugins** to enforce lifecycle rules.  
- Advanced cases: **Soulbound Tokens (SBTs)** and **Compressed NFTs (Bubblegum)**.  

---

## 🧩 Core Concepts

### Assets
- On-chain representation of a digital object (e.g., NFT).  
- Minimal by default → just metadata + ownership.  
- Extensible with plugins.  
- Can belong to a Collection or stand alone.  

### Collections
- Special Assets that **group other Assets**.  
- Carry their own metadata + plugins.  
- Enable shared rules (royalties, freezes, oracle validation).  

### Plugins & Lifecycle
Plugins = modular rules attached to Assets/Collections.  
They govern the **lifecycle**:  

1. **Create** – Initialize Asset/Collection.  
2. **Update** – Change metadata or plugin config.  
3. **Transfer** – Ownership change, rules enforced.  
4. **Freeze/Unfreeze** – Lock/unlock movement.  
5. **Burn** – Permanently remove the Asset.  

---

## 🎯 Program Features

This program exposes Anchor instructions for:  

### Collections
- Create basic Collections.  
- Add permanent transfer/freeze/burn delegates.  
- Add oracle validation.  
- Add Bubblegum plugin for compressed NFTs.  

### NFTs & Editions
- Create Master Editions and numbered Editions.  
- Mint standard NFTs.  
- Mint NFTs with permanent delegates (transfer, freeze, burn).  

### Lifecycle Ops
- Transfer NFTs.  
- Burn NFTs.  
- Update NFT metadata.  

### Plugins
- Add/update royalties.  
- Add/update freeze delegates.  
- Add autographs (support multiple).  
- Add oracle plugins.  

---

## 🔮 Special Demonstrations

1. **Soulbound Tokens (SBTs)**  
   - Permanent Freeze Delegate + Oracle Plugin = non-transferable, verifiable NFTs.  
   - Good for credentials, digital IDs, loyalty.  

2. **Bubblegum Compressed NFTs**  
   - Collections extended with Bubblegum plugin.  
   - Scalable minting (millions of NFTs via Merkle trees).  

---

## 🛠️ Prerequisites

- Rust + Anchor CLI (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli`)  
- Solana CLI (v1.18+)  
- Devnet wallet with SOL (use `solana airdrop`)  
- Node.js + Yarn  
- Basic Anchor/Solana knowledge  

---

## 🚀 Getting Started

```bash
# clone
git clone https://github.com/Worldkilas/mpl-core-examples.git
cd mpl-core-examples

# install deps
yarn install

# build & deploy
anchor build
solana config set --url devnet
anchor deploy

# run tests
anchor test

---

## 🖥️ Example Usage

### Creating a Collection
```rust
use anchor_lang::prelude::*;
use mpl_core::instructions::CreateCollectionV1;

 /// Creates a basic collection with a name and URI.
    pub fn create_collection(&mut self, args: CreateCollectionArgs) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program)
            .collection(self.collection.to_account_info().as_ref())
            .update_authority(self.update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .uri(args.uri)
            .name(args.name)
            .invoke()?;
        Ok(())
    }
```

### Minting an NFT
```rust
use anchor_lang::prelude::*;
use mpl_core::instructions::CreateAssetV1CpiBuilder;

pub fn create_nft(&mut self, create_nft_args: CreateNFTArgs) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program)
            .asset(self.asset.to_account_info().as_ref())
            .collection(self.collection.as_ref())
            .authority(self.authority.as_deref())
            .owner(self.owner.as_ref())
            .update_authority(self.update_authority.as_ref())
            .payer(self.payer.to_account_info().as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .name(create_nft_args.name)
            .uri(create_nft_args.uri)
            .invoke()?;
        Ok(())
    }
```

### Adding a Royalties Plugin
```rust
use anchor_lang::prelude::*;
use mpl_core::instructions::AddPluginV1;

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
```

For more examples, explore the `programs/` and `tests/` directories.

---

## ❓ FAQs

### What is the difference between Metaplex Core and Token Metadata?
Metaplex Core uses a **single-account design** for NFTs, reducing minting costs and complexity compared to the Token Metadata program, which relies on SPL tokens and multiple accounts. Core also offers a flexible plugin system for advanced functionality.[](https://www.quicknode.com/guides/solana-development/nfts/metaplex-core)

### How do I test on Devnet?
Set your Solana CLI to Devnet (`solana config set -ud`), fund your wallet with Devnet SOL (`solana airdrop 2`), and run `anchor deploy` followed by `anchor test`.

### Can I create large collections?
Yes! Use the **Bubblegum plugin** for compressed NFTs to mint millions of NFTs efficiently using Merkle trees.[](https://developers.metaplex.com/guides/javascript/how-to-create-an-nft-on-solana)

---

## 🔗 Resources

- [Metaplex Core Documentation](https://developers.metaplex.com/core)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [QuickNode Guide: Minting with Metaplex Core](https://www.quicknode.com/guides/solana-development/nfts/metaplex-core)
- Join the [Metaplex Discord](https://discord.com/invite/metaplex) for community support.

---

Happy building with Metaplex Core! 🚀