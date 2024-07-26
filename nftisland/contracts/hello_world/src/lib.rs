// Working
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, String, Symbol, symbol_short};

const COUNT_NFT : Symbol = symbol_short!("CNT_NFT");

#[contracttype]
pub enum Nftbook {
    Nft(u32)
}


#[contracttype]
#[derive(Clone, Debug)]
pub struct Nft {
    nft_id: u32,
    nft_owner: String,
    caption: String,
    ipfs_cid: String,
}

#[contract]
pub struct Nftisland;


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

        env.storage().instance().set(&Nftbook:: Nft(nft_details.nft_id.clone()), &nft_details);
        env.storage().instance().set(&COUNT_NFT, &nft_details.nft_id.clone());
        env.storage().instance().extend_ttl(5000, 5000);

        return nft_details.nft_id ;        
    }

    
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