use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{prelude::Uuid, EntityTrait, LoaderTrait, ModelTrait, QuerySelect};

use std::sync::Arc;

use crate::app::orm::tags::Entity as Tags;
use crate::app::{common::errors::AppError, orm::post_tags::Entity as PostTags};

use crate::app::orm::posts::{Column as PostColumn, Entity as Posts};
use crate::AppState;

use super::model::PostResponse;

pub async fn get_posts(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<PostResponse>>, AppError> {
    let posts = Posts::find()
        .columns([
            PostColumn::Id,
            PostColumn::Title,
            PostColumn::Content,
            PostColumn::UpdatedAt,
            PostColumn::Summary,
            PostColumn::CoverImage,
            PostColumn::ReadTimeMillis,
        ])
        .all(&app_state.db)
        .await?;

    let tags = posts
        .load_many_to_many(Tags, PostTags, &app_state.db)
        .await?;

    let posts_with_tags = posts
        .iter()
        .zip(tags.iter())
        .map(|(post, tags)| PostResponse::new(post, tags))
        .collect();

    Ok(Json(posts_with_tags))
}

pub async fn get_post(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<PostResponse>>, AppError> {
    let post = Posts::find_by_id(id)
        .columns([
            PostColumn::Id,
            PostColumn::Title,
            PostColumn::Content,
            PostColumn::Summary,
            PostColumn::UpdatedAt,
        ])
        .one(&app_state.db)
        .await?;

    let post = match post {
        Some(post) => post,
        None => {
            return Ok(Json(None));
        }
    };

    let tags = post
        .clone()
        .find_related(Tags)
        .all(&app_state.db)
        .await
        .unwrap();

    return Ok(Json(Some(PostResponse::new(&post, &tags))));
}
