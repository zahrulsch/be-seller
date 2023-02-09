use crate::{prelude::*, state::State, error::InvalidArgument};
use entity::{config, collection};
use sea_orm::{EntityTrait, Set};
use crate::background::{runner_keyword, RunnerKeywordData};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CrawlByKeywordsPayload {
    pub keywords: Vec<String>,
    pub thread_size: u8,
    pub name: String,
    pub limit_product: u16,
    pub config_id: i8
}

#[tauri::command]
pub async fn crawl_by_keywords(state: State<'_>, data: CrawlByKeywordsPayload) -> Result<String> {
    let CrawlByKeywordsPayload {
        config_id,
        keywords,
        limit_product,
        name,
        mut thread_size
    } = data;
    let msg_err = "terdapat argument kosong / tidak sesuai";
    let db = state.db.clone();

    if config_id < 1 { return Err(InvalidArgument::create_error("config_id", msg_err)); }
    if keywords.is_empty() { return Err(InvalidArgument::create_error("keyword_length", msg_err)); }
    if limit_product < 1 { return Err(InvalidArgument::create_error("zero_limit_product", msg_err)); }
    if name.is_empty() { return Err(InvalidArgument::create_error("empty_name", msg_err)); }
    if thread_size < 1 { thread_size = 5 }

    let Some(cfg) = config::Entity::find_by_id(config_id as i64).one(&*db).await? else {
        return Err(InvalidArgument::create_error("config not found", msg_err));
    };

    let coll = collection::ActiveModel {
        config_id: Set(cfg.id),
        name: Set(name.clone()),
        ..Default::default()   
    };

    let coll = collection::Entity::insert(coll).exec_with_returning(&*db).await?;
    
    tokio::spawn( async move {
        runner_keyword(RunnerKeywordData { 
            keywords, 
            thread_size, 
            name, 
            limit_product,
            config: cfg,
            collection: coll
        }, &db).await;
    });

    Ok("task berhasil dijalankan".into())
}