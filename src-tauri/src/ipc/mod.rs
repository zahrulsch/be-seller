mod add_config_shopee;
mod crawl_by_keywords;
mod edit_config_shopee;
mod get_col_db;
mod get_config_shopee;
mod get_data_login_bigseller;
mod get_filter_config;
mod get_product_list;
mod get_profiles;
mod login_bigseller;
mod remove_config_shopee;
mod run_migration;

pub use add_config_shopee::add_config_shopee;
pub use crawl_by_keywords::crawl_by_keywords;
pub use edit_config_shopee::edit_config_shopee;
pub use get_col_db::get_col_db;
pub use get_config_shopee::get_config_shopee;
pub use get_data_login_bigseller::get_data_login_bigseller;
pub use get_filter_config::get_filter_config;
pub use get_product_list::get_product_list;
pub use get_profiles::get_profiles;
pub use login_bigseller::login_bigseller;
pub use remove_config_shopee::remove_config_shopee;
pub use run_migration::run_migration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConfigShopeePayload {
    pub ban_keywords: Vec<String>,
    pub cities: Vec<String>,
    pub exclude_cod: bool,
    pub minimum_sold: i32,
    pub minimum_star: i32,
    pub minimum_stock: i32,
    pub name: String,
    pub price_ranges: Vec<usize>,
    pub seller_types: Vec<u32>,
    pub shippings: Vec<u32>,
    pub sort_by: String,
    pub id: i32,
    pub created_at: Option<String>,
}
