use cainome::{cairo_serde::{ByteArray, CairoSerde}, rs::abigen};
use starknet::{accounts::{ExecutionEncoding, SingleOwnerAccount}, core::{chain_id, types::{BlockId, BlockTag, Felt}}, providers::{jsonrpc::HttpTransport, JsonRpcClient, Url}, signers::{LocalWallet, SigningKey}};
use starknet_crypto::poseidon_hash_many;
abigen!(MyContract, 
    "src/contract.abi.json",
    type_aliases {
        openzeppelin_access::ownable::ownable::OwnableComponent::Event as OZEvent;
        },
    derives(Debug, Clone));


#[tokio::main]
async fn main() {

    let alice_key = starknet::signers::SigningKey::from_random();
    let alice_public_key = alice_key.verifying_key();
    let alice_public_key_felt = alice_public_key.scalar();

    let context_key = starknet::signers::SigningKey::from_random();
    let context_public_key = context_key.verifying_key();
    let context_public_key_felt = context_public_key.scalar();

    let application = Application {
        id: Felt::from_hex("0x1234567890abcdef1234567890abcdef123456789").unwrap(),
        blob: Felt::from_hex("0x1234567890abcdef1234567890abcdef123456789").unwrap(),
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

    let serialized = Request::cairo_serialize(&request);
    let hash = poseidon_hash_many(&serialized);
    let signature = alice_key.sign(&hash).unwrap();
    let signed_request = Signed {
        payload: serialized,
        signature: (signature.r, signature.s),
    };

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
    account.set_block_id(BlockId::Tag(BlockTag::Pending));
    let contract_address = Felt::from_hex("0x5929826e4646763f9fa0815d1ecba185b9b8b9b52253868c2980356ec4cc8ac").unwrap();
    let contract = MyContract::new(contract_address, account);
    let tx_hash = contract.mutate(&signed_request)
    .max_fee(1000000000000000_u128.into())
    .send()
    .await
    .expect("Call to `mutate` failed");

    println!("Tx: {:?}", tx_hash);

}
