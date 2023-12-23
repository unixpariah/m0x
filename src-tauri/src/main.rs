#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ethers::{
    prelude::*,
    signers::{coins_bip39::English, LocalWallet},
};
use hex::{decode, encode};
use lazy_static::lazy_static;
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{fs, sync::Mutex, thread};
use tauri::api::path::app_data_dir;
use tauri::{Config, Manager, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

lazy_static! {
    static ref OPEN_WALLETS: Mutex<Vec<Wallet>> = Mutex::new(Vec::new());
}

#[tauri::command]
fn open_wallet(wallet: Wallet, password: &str, app: tauri::AppHandle) -> tauri::Result<()> {
    let wallet = Wallet::decrypt(wallet, password)?;
    let mut open_wallets = OPEN_WALLETS.lock().unwrap();
    if !open_wallets.contains(&wallet) {
        open_wallets.push(wallet);
    }

    let _ = tauri::WindowBuilder::new(
        &app,
        "transaction_manager",
        tauri::WindowUrl::App("transaction_manager.html".into()),
    )
    .build();
    let transaction_manager = app.get_window("transaction_manager").unwrap();
    if let Ok(Some(monitor)) = transaction_manager.primary_monitor() {
        let size = PhysicalSize::new(monitor.size().width - 420, monitor.size().height - 30);
        transaction_manager
            .set_size(size)
            .expect("Failed to set window size");
    }
    let _ = transaction_manager.move_window(Position::TopLeft);
    transaction_manager
        .emit("update_wallet_list", &*open_wallets)
        .unwrap();
    Ok(())
}

#[tauri::command]
fn read_opened_wallets() -> Vec<Wallet> {
    let open_wallets = OPEN_WALLETS.lock().unwrap();
    (*open_wallets).clone()
}

#[tauri::command]
fn generate_wallet(key_type: &str, password: &str, length: Option<usize>, name: String) {
    match key_type {
        "private_key" => Wallet::new_pk(password, name),
        "mnemonic" => Wallet::new_seed(password, name, length.unwrap()),
        _ => unreachable!(),
    };
}

#[tauri::command]
fn import_wallet(key_type: &str, password: &str, key: &str, name: String) {
    match key_type {
        "private_key" => Wallet::import_pk(key, password, name),
        "importMnemonic" => Wallet::import_seed(key, password, name),
        _ => unreachable!(),
    };
}

#[tauri::command]
fn change_name(address: &str, name: String) {
    let config = Config::default();
    let app_data_dir_path = app_data_dir(&config)
        .unwrap()
        .join(format!("m0x/signers/{}/signer.json", &address[4..22]));
    let wallet = fs::read_to_string(&app_data_dir_path).unwrap();
    let mut wallet = serde_json::from_str::<Wallet>(&wallet).unwrap();
    wallet.name = name;
    let wallet = serde_json::to_string(&wallet).unwrap();
    fs::write(&app_data_dir_path, wallet).expect("Failed to create account");
}

#[tauri::command]
fn read_wallets() -> Vec<Wallet> {
    let config = Config::default();
    let app_data_dir_path = app_data_dir(&config).unwrap().join("m0x/signers");
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Wallet {
    pub name: String,
    pub address: Address,
    pub key: String,
}

impl Wallet {
    fn new_pk(password: &str, name: String) {
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

    fn new_seed(password: &str, name: String, length: usize) {
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

    fn import_pk(private_key: &str, password: &str, name: String) {
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

    fn import_seed(mnemonic: &str, password: &str, name: String) {
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

    fn encrypt(wallet: Self, password: &str) -> Self {
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

    fn decrypt(mut wallet: Self, password: &str) -> std::io::Result<Self> {
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

    fn output_wallet(wallet: Wallet) {
        let config = Config::default();
        let mut app_data_dir_path = app_data_dir(&config)
            .unwrap()
            .join(format!("m0x/signers/{}", encode(&wallet.address[1..10])));
        fs::create_dir_all(&app_data_dir_path).expect("Failed to create account");
        app_data_dir_path.push("signer.json");
        let wallet = serde_json::to_string(&wallet).unwrap();
        fs::write(&app_data_dir_path, wallet).expect("Failed to create account");
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let wallet_tray = app.get_window("main").unwrap();
            if let Ok(Some(monitor)) = wallet_tray.primary_monitor() {
                let size = PhysicalSize::new(400, monitor.size().height - 30);
                wallet_tray
                    .set_size(size)
                    .expect("Failed to set window size");
            }
            let _ = wallet_tray.move_window(Position::TopRight);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_wallet,
            read_wallets,
            change_name,
            import_wallet,
            open_wallet,
            read_opened_wallets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
