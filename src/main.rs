use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner,
    OnlineClient,
    PolkadotConfig,
};

use serde::{Deserialize, Serialize};
mod generics;
use crate::generics::*;
use codec::{Encode,Decode};
use elgamal_trex::elgamal::PublicKey;
use elgamal_trex::{Encryption, KeyGenerator, RawKey};
use rug::rand::RandState;

#[derive(Deserialize, Debug, Serialize,Encode,Decode)]
pub struct Cipher{
    cipher_text:String,
    difficulty:u32,
    release_block_num:u32
}

const NUMBER_AFTER_CURRENT_BLOCK: u32 = 5;
const SEND_MESSAGE:&str = "I have a dream";

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod trex_node {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::new().await?;
    // get best block from chain rpc
    let block = api.rpc().block(None).await?;
    let mut block_number= 0u32;
    // get seal struct from block header
    let mut seal:Option<Seal> = None;
    if let Some(hash) = block {
        block_number = hash.block.header.number;
        let mut digest = hash.block.header.digest;
        while let Some(item) = digest.pop() {
            if let Some(raw_seal) = item.as_seal() {
                let mut coded_seal = raw_seal.1;
                seal = Some(Seal::decode(&mut coded_seal).unwrap());
                println!("{:?}",seal);
            }
        }
    } else {
        println!("Best block hash not found.");
    }
    if let Some(seal) = seal {
        // convert raw public key to public key.
        let pubkey = PublicKey::from_raw(seal.pubkey);
        // encrypt with the public key of the fifth block after the current block
        let cipher = construct_single_cipher(&pubkey,block_number+NUMBER_AFTER_CURRENT_BLOCK,block_number);
        // encode cipher
        let cipher_encode = cipher.encode();
        // send cipher to blockchain
        send_cipher(cipher_encode).await?;
    }

    Ok(())
}

async fn send_cipher(cipher:Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let account_id = AccountKeyring::Alice.to_account_id().into();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let tx = trex_node::tx().trex_module().send_trex_data(account_id,cipher);

    let tx_submit_hash = api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await?;

    println!("TREX extrinsic {:?} submitted",tx_submit_hash);

    Ok(())
}

pub fn construct_single_cipher(pubkey: &PublicKey, release_block_num:u32, current_block_number:u32) -> Cipher{
    let mut rand = RandState::new_mersenne_twister();
    let mut pubkey = pubkey.to_owned();
    let number = (release_block_num - current_block_number) as usize;
    for _ in 0..number {
        pubkey = pubkey.yield_pubkey(&mut rand,pubkey.bit_length);
    }
    let mut rand_cipher = RandState::new_mersenne_twister();
    let cipher_text = SEND_MESSAGE.to_string().encrypt(&mut rand_cipher,&pubkey);
    Cipher{
        cipher_text,
        difficulty:pubkey.bit_length,
        release_block_num
    }
}
