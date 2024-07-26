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
