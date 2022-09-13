
use aptos_crypto::ed25519::Ed25519PrivateKey;
use aptos_crypto::ValidCryptoMaterialStringExt;
use aptin_coins::client::account::Account;
use aptin_coins::client::aptin_coin::AptinCoinsClient;
use aptin_coins::{ALICE_KEY, NODE_URL};
use aptin_coins::{BTC, ETH, SOL, USDC, USDT};


fn main() {
    let client = AptinCoinsClient::new(NODE_URL.to_string());

    // Create two accounts, Alice and Bob
    let priv_key_bytes = if let Ok(priv_key) = Ed25519PrivateKey::from_encoded_string(
        ALICE_KEY,
    ) {
        Some(Vec::from(priv_key.to_bytes()))
    } else {
        None
    };

    let mut alice = Account::new(priv_key_bytes);

    println!("Initialize Coin for address: {}", alice.address());

    println!("{}", BTC);
    client.initialize(&mut alice, BTC);

    println!("{}", SOL);
    client.initialize(&mut alice, SOL);

    println!("{}", ETH);
    client.initialize(&mut alice, ETH);

    println!("{}", USDT);
    client.initialize(&mut alice, USDT);

    println!("{}", USDC);
    client.initialize(&mut alice, USDC);

    println!("End of initialize");
}