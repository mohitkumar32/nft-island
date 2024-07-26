
# Nftisland

## Project Vision

**Nftisland** is a decentralized platform designed to facilitate the creation, management, and trading of Non-Fungible Tokens (NFTs) securely and efficiently on the Soroban blockchain. The primary goal is to empower creators and collectors by providing a user-friendly interface and robust backend infrastructure for minting, storing, and retrieving NFTs with unique identifiers and metadata stored on IPFS.

## Project Structure

### 1. Nftisland Contract

The core smart contract managing the NFT lifecycle. It includes:

- **Minting NFTs**: Create new NFTs with unique identifiers and store metadata.
- **Fetching NFTs**: Retrieve stored NFTs using their unique identifier.

### 2. Enums and Structs

- **Enums**:
  - `Nftbook`: Enum to manage different types of NFT records.

- **Structs**:
  - `Nft`: Struct to hold the NFT details including ID, owner, caption, and IPFS CID.

### 3. Storage

Using Soroban SDK storage to persist NFT data:

- **COUNT_NFT**: A counter to keep track of the number of minted NFTs.
- **NFT Data**: Storage of each NFT's details using its ID as a key.

## Code Overview

### Enums

```rust
#[contracttype]
pub enum Nftbook {
    Nft(u32)
}
```

### Structs

```rust
#[contracttype]
#[derive(Clone, Debug)]
pub struct Nft {
    nft_id: u32,
    nft_owner: String,
    caption: String,
    ipfs_cid: String,
}
```

### Contract

```rust
#[contract]
pub struct Nftisland;
```

### Contract Implementation

#### Minting an NFT

```rust
#[contractimpl]
impl Nftisland {
    pub fn mint_nft(env: Env, owr: String, nft_caption: String, ipfs_cid: String) -> u32 {
        let mut nft_count: u32 = env.storage().instance().get(&COUNT_NFT).unwrap_or(0);
            nft_count += 1;

        let mut nft_details = Self::fetch_nft(env.clone(), nft_count.clone());
        
        nft_details.nft_id = nft_count;
        nft_details.nft_owner = owr;
        nft_details.caption = nft_caption;
        nft_details.ipfs_cid = ipfs_cid;

        env.storage().instance().set(&Nftbook::Nft(nft_details.nft_id.clone()), &nft_details);
        env.storage().instance().set(&COUNT_NFT, &nft_details.nft_id.clone());
        env.storage().instance().extend_ttl(5000, 5000);

        return nft_details.nft_id ;        
    }
}
```

#### Fetching an NFT

```rust
#[contractimpl]
impl Nftisland {
    pub fn fetch_nft(env: Env, nft_id: u32) -> Nft {
        let key = Nftbook::Nft(nft_id.clone());

        env.storage().instance().get(&key).unwrap_or(Nft {
            nft_id: 0,
            nft_owner: String::from_str(&env, "Not found"),
            caption: String::from_str(&env, "Caption not found"),
            ipfs_cid: String::from_str(&env, "Invalid nft ID!"),
        })
    }
}
```
