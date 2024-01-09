use crate::{close_window, Wallet, OPEN_WALLETS};
use tauri::{Manager, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
pub async fn get_balance(wallet: Wallet) -> usize {
    let balance = Wallet::get_balance(wallet).await;
    format!("{:?}", balance).parse().unwrap()
}

#[tauri::command]
pub fn close_wallet(index: Option<usize>, app: tauri::AppHandle) {
    let mut open_wallets = OPEN_WALLETS.lock().unwrap();
    match index {
        Some(i) => {
            open_wallets.remove(i);
        }
        None => {
            open_wallets.clear();
            let _ = close_window(app);
            return;
        }
    }

    let transaction_manager = app.get_window("transaction_manager").unwrap();
    transaction_manager
        .emit("update_wallet_list", &*open_wallets)
        .unwrap();
}

#[tauri::command]
pub fn open_wallet(wallet: Wallet, password: &str, app: tauri::AppHandle) -> tauri::Result<()> {
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
pub fn read_opened_wallets() -> Vec<Wallet> {
    let open_wallets = OPEN_WALLETS.lock().unwrap();
    open_wallets.to_owned()
}

#[tauri::command]
pub fn generate_wallet(key_type: &str, password: &str, length: Option<usize>, name: String) {
    match key_type {
        "private_key" => Wallet::new_pk(password, name),
        "mnemonic" => Wallet::new_seed(password, name, length.unwrap()),
        _ => unreachable!(),
    };
}

#[tauri::command]
pub fn import_wallet(key_type: &str, password: &str, key: &str, name: String) {
    match key_type {
        "private_key" => Wallet::import_pk(key, password, name),
        "import_mnemonic" => Wallet::import_seed(key, password, name),
        _ => unreachable!(),
    };
}
