#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod background;
mod error;
mod ipc;
mod prelude;
mod state;
mod utils;

use ipc::{
    add_config_shopee, crawl_by_keywords, edit_config_shopee, get_col_db, get_config_shopee,
    get_data_login_bigseller, get_filter_config, get_product_list, get_profiles, login_bigseller,
    remove_config_shopee, run_migration,
};
use sea_orm::Database;
use state::OhMyState;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
    let Ok(connection) = Database::connect(database_url).await else {
        println!("Gagal init koneksi ke database");
        return
    };

    let state = OhMyState {
        db: Arc::new(connection),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_filter_config,
            run_migration,
            get_config_shopee,
            edit_config_shopee,
            add_config_shopee,
            remove_config_shopee,
            crawl_by_keywords,
            get_data_login_bigseller,
            login_bigseller,
            get_profiles,
            get_col_db,
            get_product_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running be seller application");
}
