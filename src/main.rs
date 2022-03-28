use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    PairSigner,
};

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

    let str = "second vec u8 message2".as_bytes();
    let hash = api
        .tx()
        .capsule_module()
        .send_capsule_data(acount_id,str.to_vec(),1023)
        .sign_and_submit(&signer)
        .await?;

    println!("Capsule extrinsic submitted: {}", hash);

    Ok(())
}


