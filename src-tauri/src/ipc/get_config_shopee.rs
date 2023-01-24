use crate::prelude::*;
use entity::config::{Entity as Config, Column};
use sea_orm::{EntityTrait, QueryOrder};
use crate::state::State;

#[tauri::command]
pub async fn get_config_shopee(state: State<'_>) -> Result<Vec<serde_json::Value>> {
    let db = state.db.clone();
    let config = Config::find()
        .order_by_desc(Column::CreatedAt)
        .into_json()
        .all(&*db)
        .await?;

    Ok(config)
}