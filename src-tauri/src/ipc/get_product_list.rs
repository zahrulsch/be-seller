use crate::{prelude::*, state::State};
use entity::{collection, collection_product};
use migration::{extension::postgres::PgExpr, JoinType};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductPayload {
    pub col_id: Option<i64>,
    pub name: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub order_by: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ProductResponse {
    pub page_count: u64,
    pub products: Vec<serde_json::Value>,
}

#[tauri::command]
pub async fn get_product_list(
    state: State<'_>,
    parameters: Option<GetProductPayload>,
) -> Result<ProductResponse> {
    let db = state.db.clone();

    let mut col_id = 0i64;
    let mut name = String::new();
    let mut page = 1i64;
    let mut page_size = 20i64;
    let mut order_by = String::from("pr_desc");

    if let Some(par) = &parameters {
        if let Some(n) = &par.name {
            name = n.clone();
        }
        if let Some(ci) = par.col_id {
            col_id = ci;
        }
        if let Some(p) = par.page {
            page = p;
        }
        if let Some(ps) = par.page_size {
            page_size = ps;
        }
        if let Some(ob) = &par.order_by {
            order_by = ob.clone();
        }
    }

    let mut products = collection_product::Entity::find()
        .column_as(collection::Column::Name, "collection_name")
        .join_rev(
            JoinType::InnerJoin,
            collection::Entity::has_many(collection_product::Entity)
                .to(collection_product::Column::CollectionId)
                .into(),
        );

    if col_id != 0 {
        products = products.filter(collection_product::Column::CollectionId.eq(col_id));
    }

    if !name.is_empty() {
        products = products.filter(
            collection_product::Column::Name
                .into_expr()
                .ilike(format!("%{}%", name)),
        )
    }

    match order_by.as_str() {
        "pr_desc" => {
            products = products.order_by_desc(collection_product::Column::PriceMin);
        }
        "pr_asc" => {
            products = products.order_by_asc(collection_product::Column::PriceMin);
        }
        "sold_desc" => {
            products = products.order_by_desc(collection_product::Column::Sold);
        }
        "sold_asc" => {
            products = products.order_by_asc(collection_product::Column::Sold);
        }
        _ => {
            products = products.order_by_desc(collection_product::Column::PriceMin);
        }
    }

    let products_paginator = products.into_json().paginate(&*db, page_size as u64);
    let products = products_paginator.fetch_page((page - 1) as u64).await?;

    let page_count = products_paginator.num_items().await?;

    Ok(ProductResponse {
        page_count,
        products,
    })
}
