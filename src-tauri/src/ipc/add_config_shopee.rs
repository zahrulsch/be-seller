use crate::prelude::*;
use crate::state::State;
use super::ConfigShopeePayload;
use entity::config::ActiveModel;
use sea_orm::{Set, ActiveModelTrait};

#[tauri::command]
pub async fn add_config_shopee(state: State<'_>, data: ConfigShopeePayload) -> Result<String> {
  let db = state.db.clone();
  let ConfigShopeePayload {
    ban_keywords, 
    cities, 
    created_at: _, 
    exclude_cod, 
    id: _, 
    minimum_sold, 
    minimum_star, 
    minimum_stock, 
    name, 
    price_ranges, 
    seller_types, 
    shippings, 
    sort_by
  } = data;
  
  let cfg = ActiveModel {
    ban_keywords: Set(ban_keywords.into()),
    cities: Set(cities.into()),
    exclude_cod: Set(exclude_cod),
    minimum_sold: Set(minimum_sold),
    minimum_stock: Set(minimum_stock),
    minimum_star: Set(minimum_star),
    name: Set(name),
    price_ranges: Set(price_ranges.into()),
    seller_types: Set(seller_types.into()),
    shippings: Set(shippings.into()),
    sort_by: Set(sort_by),
    ..Default::default()
  };

  cfg.insert(&*db).await?;
  
  Ok("config berhasil ditambahkan".into())
}