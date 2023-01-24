use auto_requester::{ShopeeFilterConfig, Shopee, UrlType, ReqMethod, CommonRequester};

use crate::prelude::*;

#[tauri::command]
pub async fn get_filter_config() -> Result<ShopeeFilterConfig> {
    let url_target = "https://mall.shopee.co.id/api/v4/search/search_filter_config?page_type=search&keyword=buku";
    let client = Shopee::new(url_target, UrlType::Product, ReqMethod::Get);
    let response = client.get_filter_config().await?;

    Ok(response)
}