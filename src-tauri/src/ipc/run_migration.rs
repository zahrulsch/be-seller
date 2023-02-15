use crate::{prelude::*, state::State};
use migration::MigratorTrait;

#[tauri::command]
pub async fn run_migration(state: State<'_>) -> Result<String> {
    let db = state.db.clone();
    migration::Migrator::up(&*db, None).await?;
    Ok("migration success".to_string())
}