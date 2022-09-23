use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner,
    OnlineClient,
    PolkadotConfig,
};

use serde::{Deserialize, Serialize};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod trex_node {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let account_id = AccountKeyring::Alice.to_account_id().into();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let ciphers = self::construct_ciphers();

    let tx = trex_node::tx().trex_module().send_trex_data(account_id,ciphers);

    let tx_submit = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let submit_event =
        tx_submit.find_first::<trex_node::balances::events::Transfer>()?;

    if let Some(event) = submit_event {
        println!("Balance transfer success {:?}",event);
    } else {
        println!("Failed to find Balances::Transfer Event");
    }

    println!("Capsule extrinsic submitted");

    Ok(())
}

fn construct_ciphers() -> Vec<u8>{
    let cipher_text = "second vec u8 message".as_bytes().to_vec();
    let cipher = Cipher{
        cipher_text,
        difficulty:32,
        release_block_num:90
    };

    let cipher_text1 = "second vec u8 message".as_bytes().to_vec();
    let cipher1 = Cipher{
        cipher_text:cipher_text1,
        difficulty:32,
        release_block_num:92
    };

    let mut trasactions = vec![];
    trasactions.push(cipher);
    trasactions.push(cipher1);

    let transcode = serde_json::to_string(&trasactions).unwrap_or("".to_string());
    println!("transcode: {:?}", transcode);
    transcode.as_bytes().to_vec()
}

#[derive(Deserialize, Debug, Serialize)]
struct Cipher{
    cipher_text:Vec<u8>,
    difficulty:u32,
    release_block_num:u32
}
