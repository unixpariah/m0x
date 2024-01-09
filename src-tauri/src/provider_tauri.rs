use crate::PROVIDERS;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::api::path::app_data_dir;
use tauri::Config;

#[derive(Deserialize, Serialize, Clone)]
pub struct Provider {
    pub url: String,
    pub name: String,
}

#[tauri::command]
pub fn update_provider_list(updated_providers: Vec<Provider>) {
    let mut providers = PROVIDERS.lock().unwrap();
    *providers = updated_providers;
    let config = Config::default();
    let providers_path = app_data_dir(&config).unwrap().join("m0x/providers.json");
    let providers = serde_json::to_string(&*providers).unwrap();
    fs::write(providers_path, providers).unwrap();
}

#[tauri::command]
pub async fn get_providers() -> Vec<Provider> {
    let mut providers = PROVIDERS.lock().unwrap();
    let config = Config::default();
    let providers_path = app_data_dir(&config).unwrap().join("m0x/providers.json");
    let json_string = match fs::read_to_string(&providers_path) {
        Ok(data) => data,
        Err(_) => {
            let provider = Provider {
                url: "https://ethereum-goerli.publicnode.com".to_string(),
                name: "Default provider".to_string(),
            };
            providers.push(provider);
            fs::write(providers_path, serde_json::to_string(&*providers).unwrap()).unwrap();
            return providers.to_owned();
        }
    };
    *providers = serde_json::from_str(&json_string).unwrap();
    providers.to_owned()
}
