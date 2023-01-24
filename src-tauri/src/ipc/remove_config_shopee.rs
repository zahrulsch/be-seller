use crate::prelude::*;
use entity::config::Entity;
use migration::DbErr;
use sea_orm::{EntityTrait, ModelTrait};
use crate::state::State;
use crate::error::OhMyError;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RemoveConfigShopeePayload {
    pub id: i32
}

#[tauri::command]
pub async fn remove_config_shopee(state: State<'_>, data: RemoveConfigShopeePayload) -> Result<String> {
    let db = state.db.clone();
    let Some(cfg) = Entity::find_by_id(data.id as i64).one(&*db).await? else {
        return Err(OhMyError::DB(DbErr::RecordNotFound("config not found".into())))
    };

    cfg.delete(&*db).await?;

    Ok("config berhasil dihapus".into())
}
