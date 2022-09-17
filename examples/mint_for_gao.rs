use aptos_crypto::ed25519::Ed25519PrivateKey;
use aptos_crypto::ValidCryptoMaterialStringExt;
use aptin_coins::client::account::Account;
use aptin_coins::client::aptin_coin::AptinCoinsClient;
use aptin_coins::{BTC, NODE_URL, ETH, SOL, USDC, USDT, ALICE_KEY};

fn main() {
    let client = AptinCoinsClient::new(NODE_URL.to_string());

    let priv_key_bytes = if let Ok(priv_key) = Ed25519PrivateKey::from_encoded_string(
        ALICE_KEY,
    ) {
        Some(Vec::from(priv_key.to_bytes()))
    } else {
        None
    };
    let mut alice = Account::new(priv_key_bytes);

    let bob: &str = "f08b8a241fa57711075213a6053399bfffeab0f08838092d934096afbcc7444e";

    let amount = 100000000000000;

    println!("alice: {}", alice.address());

    let mut tx_hash = client.mint(&mut alice, bob, amount, BTC);
    client.rest_client.wait_for_transaction(&tx_hash);

    tx_hash = client.mint(&mut alice, bob, amount, ETH);
    client.rest_client.wait_for_transaction(&tx_hash);

    tx_hash = client.mint(&mut alice, bob, amount, SOL);
    client.rest_client.wait_for_transaction(&tx_hash);

    tx_hash = client.mint(&mut alice, bob, amount, USDC);
    client.rest_client.wait_for_transaction(&tx_hash);

    tx_hash = client.mint(&mut alice, bob, amount, USDT);
    client.rest_client.wait_for_transaction(&tx_hash);
}