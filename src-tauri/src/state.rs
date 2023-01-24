#![allow(dead_code)]

use sea_orm::DatabaseConnection;
use tauri::State as TauriState;
use std::sync::Arc;

#[derive(Debug)]
pub struct OhMyState {
  pub db: Arc<DatabaseConnection>
}
pub type State<'a> = TauriState<'a, OhMyState>;
