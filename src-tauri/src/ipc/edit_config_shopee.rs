use crate::{prelude::*, state::State, error};
use entity::config::{Entity as Config, ActiveModel};
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use super::ConfigShopeePayload;

#[tauri::command]
pub async fn edit_config_shopee(state: State<'_>, data: ConfigShopeePayload) -> Result<String> {
    let db = state.db.clone();
    let Some(cfg)= Config::find_by_id(data.id as i64).one(&*db).await? else {
        return Err(error::OhMyError::DB(sea_orm::DbErr::RecordNotFound("config not found".to_string())));
    };

    let mut cfg: ActiveModel = cfg.into();

    cfg.name = Set(data.name);
    cfg.ban_keywords = Set(data.ban_keywords.into());
    cfg.cities = Set(data.cities.into());
    cfg.exclude_cod = Set(data.exclude_cod);
    cfg.price_ranges = Set(data.price_ranges.into());
    cfg.shippings = Set(data.shippings.into());
    cfg.sort_by = Set(data.sort_by);
    cfg.minimum_sold = Set(data.minimum_sold);
    cfg.minimum_star = Set(data.minimum_star);
    cfg.minimum_stock = Set(data.minimum_stock);
    cfg.seller_types = Set(data.seller_types.into());

    cfg.update(&*db).await?;

    Ok("update config berhasil".into())
}