pub fn init(account_addr: &str, coin_type: &str) -> serde_json::Value {
    match coin_type {
        "BTC" => {
            serde_json::json!({
                "type": "entry_function_payload",
                "function": "0x1::managed_coin::initialize",
                "type_arguments": [format!("0x{}::coins::BTC", account_addr)],
                "arguments": [
                    hex::encode("Bit Coin".as_bytes()),
                    hex::encode("BTC".as_bytes()),
                    0,
                    false,
                ]
            })
        }
        "ETH" => {
            serde_json::json!({
                "type": "entry_function_payload",
                "function": "0x1::managed_coin::initialize",
                "type_arguments": [format!("0x{}::coins::ETH", account_addr)],
                "arguments": [
                    hex::encode("Ethereum Coin".as_bytes()),
                    hex::encode("ETH".as_bytes()),
                    0,
                    false,
                ]
            })
        }
        "USDC" => {
            serde_json::json!({
                "type": "entry_function_payload",
                "function": "0x1::managed_coin::initialize",
                "type_arguments": [format!("0x{}::coins::USDC", account_addr)],
                "arguments": [
                    hex::encode("USD Coin".as_bytes()),
                    hex::encode("USDC".as_bytes()),
                    0,
                    false,
                ]
            })
        }
        "USDT" => {
            serde_json::json!({
                "type": "entry_function_payload",
                "function": "0x1::managed_coin::initialize",
                "type_arguments": [format!("0x{}::coins::USDT", account_addr)],
                "arguments": [
                    hex::encode("Tether".as_bytes()),
                    hex::encode("USDT".as_bytes()),
                    0,
                    false,
                ]
            })
        }
        "SOL" => {
            serde_json::json!({
                "type": "entry_function_payload",
                "function": "0x1::managed_coin::initialize",
                "type_arguments": [format!("0x{}::coins::SOL", account_addr)],
                "arguments": [
                    hex::encode("Solana Coin".as_bytes()),
                    hex::encode("SOL".as_bytes()),
                    0,
                    false,
                ]
            })
        }
        _ => panic!("Not support coin"),
    }
}
