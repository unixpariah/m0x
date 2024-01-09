#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod eth;
mod provider_tauri;
mod wallet_tauri;

use eth::Wallet;
use ethers::prelude::*;
use lazy_static::lazy_static;
use provider_tauri::{get_providers, update_provider_list};
use std::{fs, sync::Mutex, thread};
use tauri::api::path::app_data_dir;
use tauri::{Config, Manager, PhysicalSize};
use wallet_tauri::{
    close_wallet, generate_wallet, get_balance, import_wallet, open_wallet, read_opened_wallets,
};

lazy_static! {
    static ref OPEN_WALLETS: Mutex<Vec<Wallet>> = Mutex::new(Vec::new());
    pub static ref PROVIDERS: Mutex<Vec<provider_tauri::Provider>> = Mutex::new(Vec::new());
}

#[tauri::command]
async fn get_data() -> (String, String) {
    let url = PROVIDERS.lock().unwrap().to_owned();
    let provider = Provider::<Http>::connect(&url[0].url).await;
    let gas_price = provider
        .get_gas_price()
        .await
        .unwrap_or(U256::from(0))
        .to_string();
    let block_number = provider
        .get_block_number()
        .await
        .unwrap_or(U64::from(0))
        .to_string();
    (gas_price, block_number)
}

#[tauri::command]
fn close_window(app: tauri::AppHandle) -> tauri::Result<()> {
    if let Some(transaction_manager) = app.get_window("transaction_manager") {
        transaction_manager.close()?;
    }
    Ok(())
}

/*
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
*/

#[tauri::command]
fn read_wallets() -> Vec<Wallet> {
    let config = Config::default();
    let signers_path = app_data_dir(&config).unwrap().join("m0x/signers");
    let _ = fs::create_dir_all(&signers_path);
    let wallets = fs::read_dir(&signers_path).unwrap();
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

#[tokio::main]
async fn main() {
    get_providers().await;
    tauri::Builder::default()
        .setup(|app| {
            let wallet_tray = app.get_window("main").unwrap();
            if let Ok(Some(monitor)) = wallet_tray.primary_monitor() {
                let size = PhysicalSize::new(400, monitor.size().height - 30);
                wallet_tray
                    .set_size(size)
                    .expect("Failed to set window size");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_wallet,
            read_wallets,
            import_wallet,
            open_wallet,
            read_opened_wallets,
            close_wallet,
            get_balance,
            get_data,
            update_provider_list,
            get_providers,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
