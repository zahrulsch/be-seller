#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod prelude;
mod error;
mod state;
mod ipc;

use std::sync::Arc;
use sea_orm::Database;
use state::OhMyState;
use ipc::{
    edit_config_shopee, 
    add_config_shopee, 
    remove_config_shopee, 
    run_migration, 
    get_config_shopee,
    get_filter_config
};

#[tokio::main]
async fn main() {
    let Ok(connection) = Database::connect("postgres://postgres:postgres@localhost:5432/postgres" ).await else {
        println!("Gagal init koneksi ke database");
        return
    };

    let state = OhMyState {
        db: Arc::new(connection)
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_filter_config,
            run_migration,
            get_config_shopee,
            edit_config_shopee,
            add_config_shopee,
            remove_config_shopee
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
