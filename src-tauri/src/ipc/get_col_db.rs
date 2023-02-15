use crate::{prelude::*, state::State};
use entity::{collection, collection_product};
use sea_orm::{ColumnTrait, EntityTrait, QuerySelect, QueryOrder, JoinType};

#[tauri::command]
pub async fn get_col_db(state: State<'_>) -> Result<Vec<serde_json::Value>> {
    let db = state.db.clone();

    let collections = collection::Entity::find()
        .column_as(
            collection_product::Column::CollectionId.count(),
            "product_count",
        )
        .column_as(
            collection_product::Column::PriceMin.max(),
            "max_price_product",
        )
        .column_as(
            collection_product::Column::PriceMin.min(),
            "min_price_product",
        )
        .join_rev(
            JoinType::InnerJoin,
            collection_product::Entity::belongs_to(collection::Entity)
                .from(collection_product::Column::CollectionId)
                .to(collection::Column::Id)
                .into(),
        )
        .group_by(collection::Column::Id)
        .order_by_desc(collection::Column::CreatedAt)
        .into_json()
        .all(&*db)
        .await?;

    Ok(collections)
}
