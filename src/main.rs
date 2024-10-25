use std::str::FromStr;

use cainome::cairo_serde::ByteArray;
use starknet_accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet_core::chain_id;
use starknet_providers::jsonrpc::HttpTransport;
use starknet_providers::{JsonRpcClient, Url};
use starknet_core::types::Felt;
use starknet_signers::{LocalWallet, SigningKey};
use starknet_crypto::poseidon_hash_many;

mod types;
use crate::types::*;

use cainome::rs::abigen;
abigen!(ContextConfig, "contract/abi.json", output_path("src/types.rs"));

#[tokio::main] 
async fn main() {

    let contract_id = "0x67e4410cd0cbb252a0b4bca0ec4f89e40bc7ff64ba6121ad447d9ac2ed28bdb";

    let alice_key = starknet_signers::SigningKey::from_random();
    let alice_public_key = alice_key.verifying_key();
    let alice_public_key_felt = alice_public_key.scalar();

    let context_key = starknet_signers::SigningKey::from_random();
    let context_public_key = context_key.verifying_key();
    let context_public_key_felt = context_public_key.scalar();

    let application = Application {
        id: Felt::from_str("0x1234567890abcdef1234567890abcdef123456789").unwrap(),
        blob: Felt::from_str("0x1234567890abcdef1234567890abcdef123456789").unwrap(),
        size: 0,
        source: ByteArray::from_string("https://calimero.network").unwrap(),
        metadata: ByteArray::from_string("Some metadata").unwrap(),
    };

    let request = Request {
        signer_id: alice_public_key_felt,
        nonce: 0,
        kind: RequestKind::Context(ContextRequest {
            context_id: context_public_key_felt,
            kind: ContextRequestKind::Add((context_public_key_felt, application.clone())),
        }),
    };
    let mut serialized = vec![];
    // let _ = request.serialize(&mut serialized);
    let hash = poseidon_hash_many(&serialized);
    let signature = alice_key.sign(&hash).unwrap();
    let signed_request = Signed {
        payload: serialized,
        signature: (signature.r, signature.s),
    };
    // let mut signed_request_serialized = vec![];
    // let _ = signed_request
    //     .encode(&mut signed_request_serialized)
    //     .unwrap();

    //Call the contract
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let wallet_pk = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x3466a2196c72a94edd80c49baaad89c3cd71815038c31e1d3c94337ad97406d")
            .unwrap(),
    ));
    let wallet_address =
        Felt::from_hex("0x55d0c6f18991cebbf0dee233cafdd19f906f387a364b040c71c00094355e281")
            .unwrap();
    let mut account = SingleOwnerAccount::new(
        provider,
        wallet_pk,
        wallet_address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    let contract = ContextConfig::new(contract_id, account);

    let response = contract.mutate(request).send().await.unwrap();
    println!("{:?}", response);
    // println!("Account {:?}", account.address());
    // account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // let result = account
    //   .execute_v1(vec![Call {
    //       to: Felt::from_str(contract_id).unwrap(),
    //       selector: Felt::from_str("0x33f8af2c6d5b2376345a3c43ad230b0741fb5694c7064741c1927142bbd442a").unwrap(),
    //       calldata: signed_request_serialized,
    //   }])
    //   .send()
    //   .await
    //   .unwrap();
    
    // println!("{:?}", result);

}
