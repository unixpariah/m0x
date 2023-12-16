#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bip39::{Language, Mnemonic, MnemonicType};
use ethers::{
    prelude::*,
    signers::{LocalWallet, Wallet},
};
use hex::encode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tauri::{Manager, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
fn greet(name: &str) -> String {
    Wallet::new_seed();
    String::new()
}

#[derive(Debug, Serialize, Deserialize)]
struct Wallet {
    address: Address,
    key: String,
}

impl Wallet {
    fn new_pk() -> Self {
        let mut rng = rand::thread_rng();

        let private_key: [u8; 32] = rng.gen();
        let private_key = encode(private_key);
        let wallet: LocalWallet = private_key.parse().unwrap();

        Wallet {
            address: wallet.address(),
            key: private_key,
        }
    }

    fn new_seed() {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        println!("{:#?}", mnemonic);
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
