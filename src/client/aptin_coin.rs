#![allow(dead_code)]

use super::{account::Account, coins, rest::RestClient};

pub struct AptinCoinsClient {
    pub rest_client: RestClient,
}

impl AptinCoinsClient {
    /// Represents an account as well as the private, public key-pair for the Aptos blockchain.
    pub fn new(url: String) -> Self {
        Self {
            rest_client: RestClient::new(url),
        }
    }

    //:!:>section_1
    /// Initializes the new coin.
    pub fn initialize(&self, account_from: &mut Account, coin_type: &str) -> String {
        let payload = coins::init(&account_from.address(), coin_type);
        println!("init {}: {}", coin_type, payload);
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }
    //<:!:section_1

    //:!:>section_2
    /// Receiver needs to register the coin before they can receive it.
    pub fn register(
        &self,
        account_receiver: &mut Account,
        coin_type_address: &str,
        coin_type: &str,
    ) -> String {
        let payload = serde_json::json!({
            "type": "entry_function_payload",
            "function": "0x1::managed_coin::register",
            "type_arguments": [format!("0x{}::coins::{}", coin_type_address, coin_type)],
            "arguments": []
        });
        self.rest_client
            .execution_transaction_with_payload(account_receiver, payload)
    }
    //<:!:section_2

    //:!:>section_3
    /// Receiver needs to register the coin before they can receive it.
    pub fn mint(
        &self,
        admin: &mut Account,
        receiver_address: &str,
        amount: u64,
        coin_type: &str,
    ) -> String {
        let payload = serde_json::json!({
            "type": "entry_function_payload",
            "function": "0x1::managed_coin::mint",
            "type_arguments": [format!("0x{}::coins::{}", admin.address(), coin_type)],
            "arguments": [
                receiver_address,
                // amount,
                amount.to_string(),
            ]
        });
        println!("payload: {}", payload);
        self.rest_client
            .execution_transaction_with_payload(admin, payload)
    }
    //<:!:section_3

    //:!:>section_4
    /// Receiver needs to register the coin before they can receive it.
    pub fn get_balance(
        &self,
        account_address: &str,
        coin_type_address: &str,
        coin_type: &str,
    ) -> u64 {
        let module_type = format!(
            "0x1::coin::CoinStore<0x{}::coins::{}>",
            coin_type_address, coin_type
        );
        self.rest_client
            .account_resource(account_address, &module_type)
            .map(|value| {
                // println!("value: {:?}", value);
                value["data"]["coin"]["value"]
                    .as_str()
                    .unwrap()
                    .to_string()
                    .parse::<u64>()
                    .unwrap()
            })
            .unwrap()
    }
    //<:!:section_4
}
