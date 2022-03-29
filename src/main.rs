use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    PairSigner,
};

use serde::{Deserialize, Serialize};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let acount_id = AccountKeyring::Alice.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let str = self::construct_data();

    let hash = api
        .tx()
        .capsule_module()
        .send_capsule_data(acount_id,str)
        .sign_and_submit(&signer)
        .await?;

    println!("Capsule extrinsic submitted: {:?}", hash);

    Ok(())
}

fn construct_data() -> Vec<u8>{
    let cipher_text = "second vec u8 message".as_bytes().to_vec();
    let cipher = Cipher{
        cipher_text,
        difficulty:32,
        release_block_num:90
    };

    let cipher_text1 = "second vec u8 message1".as_bytes().to_vec();
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


