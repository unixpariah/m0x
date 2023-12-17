#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ethers::{
    prelude::*,
    signers::{coins_bip39::English, LocalWallet},
};
use hex::encode;
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use tauri::{Manager, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
fn generate_wallet(key_type: &str, password: &str, length: Option<usize>) -> Wallet {
    match key_type {
        "private_key" => Wallet::new_pk(password),
        "mnemonic" => Wallet::new_seed(length.unwrap()),
        _ => unreachable!(),
    }
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

        let password = {
            let mut sha256 = sha2::Sha256::new();
            sha256.update(password.as_bytes());
            &hex::encode(sha256.finalize())[..16]
        };

        let cipher = Cipher::aes_128_ecb();
        let encrypted_wallets = encrypt(cipher, password.as_bytes(), None, private_key.as_bytes())
            .expect("Failed to encrypt wallets");
        //TODO: make it output into file

        Wallet {
            address: wallet.address(),
            key: private_key,
        }
    }

    fn new_seed(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = coins_bip39::Mnemonic::<English>::new_with_count(&mut rng, length).unwrap();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase::<PathOrString>(mnemonic.to_phrase().as_str().into())
            .build()
            .unwrap();

        Wallet {
            address: wallet.address(),
            key: mnemonic.to_phrase(),
        }
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
        .invoke_handler(tauri::generate_handler![generate_wallet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
