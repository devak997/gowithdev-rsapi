use std::sync::Arc;

use crate::app::common::core::AppState;
use crate::app::common::errors::AppError;
use crate::app::orm::post_categories::Entity as PostCategories;
use crate::app::orm::post_categories::Model as PostCategory;
use axum::{extract::State, Json};
use sea_orm::EntityTrait;

pub async fn get_categories(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<PostCategory>>, AppError> {
    let categories = PostCategories::find().all(&app_state.db).await?;
    Ok(Json(categories))
}
