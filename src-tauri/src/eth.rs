use crate::read_wallets;
use ethers::{
    prelude::*,
    signers::{coins_bip39::English, LocalWallet},
};
use hex::{decode, encode};
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{fs, sync::Arc};
use tauri::api::path::app_data_dir;
use tauri::Config;

pub const PROVIDER: &str = "https://rpc-goerli.flashbots.net/";

abigen!(
    Balance,
    "./contracts/balance_scanner.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Wallet {
    pub name: String,
    address: Address,
    key: String,
}

impl Wallet {
    pub fn new_pk(password: &str, name: String) {
        let mut rng = rand::thread_rng();
        let private_key: [u8; 32] = rng.gen();
        let private_key = encode(private_key);
        let wallet: LocalWallet = private_key.parse().unwrap();

        let name = if name.trim() == "" {
            format!("Wallet {}", read_wallets().len() + 1)
        } else {
            name
        };

        let wallet = Wallet {
            name,
            address: wallet.address(),
            key: private_key,
        };

        let encrypted_wallet = Self::encrypt(wallet, password);
        Self::output_wallet(encrypted_wallet);
    }

    pub fn new_seed(password: &str, name: String, length: usize) {
        let mut rng = rand::thread_rng();
        let mnemonic = coins_bip39::Mnemonic::<English>::new_with_count(&mut rng, length).unwrap();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase::<PathOrString>(mnemonic.to_phrase().as_str().into())
            .build()
            .unwrap();

        let name = if name.is_empty() {
            format!("Wallet {}", read_wallets().len() + 1)
        } else {
            name
        };
        let wallet = Wallet {
            name,
            address: wallet.address(),
            key: mnemonic.to_phrase(),
        };

        let encrypted_wallet = Self::encrypt(wallet, password);
        Self::output_wallet(encrypted_wallet);
    }

    pub fn import_pk(private_key: &str, password: &str, name: String) {
        let wallet: LocalWallet = match private_key.parse() {
            Ok(wallet) => wallet,
            Err(_) => return,
        };

        let name = if name.is_empty() {
            format!("Wallet {}", read_wallets().len() + 1)
        } else {
            name
        };

        let wallet = Wallet {
            name,
            address: wallet.address(),
            key: private_key.to_string(),
        };

        let encrypted_wallet = Self::encrypt(wallet, password);
        Self::output_wallet(encrypted_wallet);
    }

    pub fn import_seed(mnemonic: &str, password: &str, name: String) {
        let wallet = match MnemonicBuilder::<English>::default()
            .phrase::<PathOrString>(mnemonic.into())
            .build()
        {
            Ok(wallet) => wallet,
            Err(_) => return,
        };

        let name = if name.is_empty() {
            format!("Wallet {}", read_wallets().len() + 1)
        } else {
            name
        };
        let wallet = Wallet {
            name,
            address: wallet.address(),
            key: mnemonic.to_string(),
        };

        let encrypted_wallet = Self::encrypt(wallet, password);
        Self::output_wallet(encrypted_wallet);
    }

    pub fn encrypt(wallet: Self, password: &str) -> Self {
        let password = {
            let mut sha256 = sha2::Sha256::new();
            sha256.update(password.as_bytes());
            &hex::encode(sha256.finalize())[..16]
        };

        let encrypted_key = encrypt(
            Cipher::aes_128_ecb(),
            password.as_bytes(),
            None,
            wallet.key.as_bytes(),
        )
        .expect("Failed to encrypt wallets");

        let name = if wallet.name.is_empty() {
            format!("Wallet {}", read_wallets().len() + 1)
        } else {
            wallet.name
        };
        let wallet = Wallet {
            name,
            address: wallet.address,
            key: encode(encrypted_key.clone()),
        };

        wallet
    }

    pub fn decrypt(mut wallet: Self, password: &str) -> std::io::Result<Self> {
        let password = {
            let mut sha256 = sha2::Sha256::new();
            sha256.update(password.as_bytes());
            &hex::encode(sha256.finalize())[..16]
        };

        let decrypted_key = decrypt(
            Cipher::aes_128_ecb(),
            password.as_bytes(),
            None,
            &decode(wallet.key).unwrap(),
        )?;
        let key = String::from_utf8(decrypted_key).unwrap();

        wallet.key = key;
        Ok(wallet)
    }

    pub fn output_wallet(wallet: Wallet) {
        let config = Config::default();
        let mut app_data_dir_path = app_data_dir(&config)
            .unwrap()
            .join(format!("m0x/signers/{}", encode(&wallet.address[1..10])));
        fs::create_dir_all(&app_data_dir_path).expect("Failed to create account");
        app_data_dir_path.push("signer.json");
        let wallet = serde_json::to_string(&wallet).unwrap();
        fs::write(&app_data_dir_path, wallet).expect("Failed to create account");
    }

    pub async fn get_balance(wallet: Wallet) -> U256 {
        let provider = Provider::<Http>::connect(PROVIDER).await;
        let balance_scanner_address = "0x9788C4E93f9002a7ad8e72633b11E8d1ecd51f9b"
            .parse::<Address>()
            .unwrap();

        let balance_scanner = Balance::new(balance_scanner_address, Arc::new(provider.clone()));

        let zero_address = vec!["0x0000000000000000000000000000000000000000"
            .parse::<Address>()
            .unwrap()];

        match balance_scanner
            .balances(vec![wallet.address], zero_address)
            .call()
            .await
        {
            Ok(balances) => balances[0],
            Err(_) => U256::from(0),
        }
    }
}
