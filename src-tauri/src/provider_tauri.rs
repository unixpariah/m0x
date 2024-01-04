use crate::PROVIDERS;
use std::fs;
use tauri::api::path::app_data_dir;
use tauri::Config;

#[tauri::command]
pub fn update_provider_list(provider: String) {
    let mut providers = PROVIDERS.lock().unwrap();
    if providers.contains(&provider) {
        return;
    }
    providers.push(provider);
    let config = Config::default();
    let providers_path = app_data_dir(&config).unwrap().join("m0x/providers.json");
    let providers = serde_json::to_string(&*providers).unwrap();
    fs::write(providers_path, providers).unwrap();
}

#[tauri::command]
pub async fn get_providers() -> Vec<String> {
    let mut providers = PROVIDERS.lock().unwrap();
    let config = Config::default();
    let providers_path = app_data_dir(&config).unwrap().join("m0x/providers.json");
    match fs::read_to_string(&providers_path) {
        Ok(json_string) => {
            *providers = serde_json::from_str(json_string.as_str()).unwrap();
        }
        Err(_) => {
            providers.push("https://ethereum-goerli.publicnode.com".to_string());
            fs::write(providers_path, serde_json::to_string(&*providers).unwrap()).unwrap();
        }
    };
    providers.to_owned()
}
