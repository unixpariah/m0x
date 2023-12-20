#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ethers::{
    prelude::*,
    signers::{coins_bip39::English, LocalWallet},
};
use hex::encode;
use openssl::symm::{encrypt, Cipher};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{fs, thread};
use tauri::api::path::app_data_dir;
use tauri::{Config, Manager, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
fn generate_wallet(key_type: &str, password: &str, length: Option<usize>) -> Wallet {
    match key_type {
        "private_key" => Wallet::new_pk(password),
        "mnemonic" => Wallet::new_seed(password, length.unwrap()),
        _ => unreachable!(),
    }
}

#[tauri::command]
fn read_wallets() -> Vec<Wallet> {
    let config = Config::default();
    let mut app_data_dir_path = app_data_dir(&config).unwrap();
    app_data_dir_path.push("m0x/signers");
    let wallets = fs::read_dir(&app_data_dir_path).unwrap();
    let mut handles = Vec::new();
    wallets.into_iter().for_each(|signer| {
        let handle = thread::spawn(|| {
            let wallet = fs::read_to_string(signer.unwrap().path().join("signer.json")).unwrap();
            serde_json::from_str(&wallet).unwrap()
        });
        handles.push(handle)
    });

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct Wallet {
    pub address: Address,
    pub key: String,
}

impl Wallet {
    fn new_pk(password: &str) -> Self {
        let mut rng = rand::thread_rng();
        let private_key: [u8; 32] = rng.gen();
        let private_key = encode(private_key);
        let wallet: LocalWallet = private_key.parse().unwrap();

        let wallet = Wallet {
            address: wallet.address(),
            key: private_key,
        };

        let encrypted_wallet = Self::encrypt(&wallet, password);
        Self::output_wallet(encrypted_wallet);

        wallet
    }

    fn new_seed(password: &str, length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = coins_bip39::Mnemonic::<English>::new_with_count(&mut rng, length).unwrap();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase::<PathOrString>(mnemonic.to_phrase().as_str().into())
            .build()
            .unwrap();

        let wallet = Wallet {
            address: wallet.address(),
            key: mnemonic.to_phrase(),
        };

        let encrypted_wallet = Self::encrypt(&wallet, password);
        Self::output_wallet(encrypted_wallet);

        wallet
    }

    fn encrypt(wallet: &Wallet, password: &str) -> Wallet {
        let password = {
            let mut sha256 = sha2::Sha256::new();
            sha256.update(password.as_bytes());
            &hex::encode(sha256.finalize())[..16]
        };

        let cipher = Cipher::aes_128_ecb();
        let encrypted_key = encrypt(cipher, password.as_bytes(), None, wallet.key.as_bytes())
            .expect("Failed to encrypt wallets");

        let wallet = Wallet {
            address: wallet.address,
            key: encode(encrypted_key),
        };

        wallet
    }

    fn output_wallet(wallet: Wallet) {
        let config = Config::default();
        let mut app_data_dir_path = app_data_dir(&config).unwrap();
        app_data_dir_path.push(format!("m0x/signers/{}", encode(&wallet.address[1..10])));
        fs::create_dir_all(&app_data_dir_path).expect("Failed to create account");
        app_data_dir_path.push("signer.json");
        let wallet = serde_json::to_string(&wallet).unwrap();
        fs::write(&app_data_dir_path, wallet).expect("Failed to create account");
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::RightCenter);

            let main_window = app.get_window("main").unwrap();
            if let Ok(Some(monitor)) = main_window.primary_monitor() {
                let size = PhysicalSize::new(400, monitor.size().height - 55);
                main_window
                    .set_size(size)
                    .expect("Failed to set window size");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_wallet, read_wallets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
