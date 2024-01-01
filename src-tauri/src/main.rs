#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod eth;

use eth::{Wallet, PROVIDER};
use ethers::prelude::*;
use lazy_static::lazy_static;
use std::{fs, sync::Mutex, thread};
use tauri::api::path::app_data_dir;
use tauri::{Config, Manager, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

lazy_static! {
    static ref OPEN_WALLETS: Mutex<Vec<Wallet>> = Mutex::new(Vec::new());
}

#[tauri::command]
async fn get_data() -> (String, String, String) {
    let provider = Provider::<Http>::connect(PROVIDER).await;
    (
        provider
            .get_chainid()
            .await
            .unwrap_or(U256::from(1))
            .to_string(),
        provider
            .get_gas_price()
            .await
            .unwrap_or(U256::from(0))
            .to_string(),
        provider
            .get_block_number()
            .await
            .unwrap_or(U64::from(0))
            .to_string(),
    )
}

#[tauri::command]
async fn get_balance(wallet: Wallet) -> usize {
    let balance = Wallet::get_balance(wallet).await;
    format!("{:?}", balance).parse().unwrap()
}

#[tauri::command]
fn close_wallet(index: Option<usize>, app: tauri::AppHandle) {
    let mut open_wallets = OPEN_WALLETS.lock().unwrap();
    match index {
        Some(i) => {
            open_wallets.remove(i);
        }
        None => open_wallets.clear(),
    }

    let transaction_manager = app.get_window("transaction_manager").unwrap();
    transaction_manager
        .emit("update_wallet_list", &*open_wallets)
        .unwrap();
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
        tauri::WindowUrl::App("transactionManager.html".into()),
    )
    .build();
    let transaction_manager = app.get_window("transaction_manager").unwrap();
    if let Ok(Some(monitor)) = transaction_manager.primary_monitor() {
        let size = PhysicalSize::new(monitor.size().width - 420, monitor.size().height - 30);
        let _ = transaction_manager.set_size(size);
        let _ = transaction_manager.set_title("Transaction Manager");
        let _ = transaction_manager.move_window(Position::TopLeft);
        let _ = transaction_manager.emit("update_wallet_list", &*open_wallets);
    }
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
        "import_mnemonic" => Wallet::import_seed(key, password, name),
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
    let _ = fs::create_dir_all(&app_data_dir_path);
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
            read_opened_wallets,
            close_wallet,
            get_balance,
            get_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
