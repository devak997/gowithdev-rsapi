use std::str::FromStr;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{FromRequest, Path, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Extension, Json, RequestExt};
use sea_orm::prelude::Uuid;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, ModelTrait, QueryFilter,
};

use crate::app::auth::model::UserInfo;
use crate::app::common::core::AppState;
use crate::app::common::errors::AppError;
use crate::app::orm::post_tags::{
    ActiveModel as PostTagActiveModel, Column as PostTagColumn, Entity as PostTags,
};
use crate::app::orm::posts::{ActiveModel as PostActiveModel, Entity as Posts, Model as Post};
use crate::app::orm::tags::{
    ActiveModel as TagActiveModel, Column as TagColumn, Entity as Tags, Model as Tag,
};

use super::model::{ModifyPostRequest, PostResponse};

const POST_NOT_FOUND_ERROR: &str = "Post not found";

#[async_trait]
impl<S> FromRequest<S> for ModifyPostRequest
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, _: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
        Ok(payload)
    }
}

pub async fn get_post(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, AppError> {
    let post = Posts::find_by_id(id).one(&app_state.db).await?;

    let post = match post {
        Some(post) => post,
        None => {
            return Err(AppError {
                status: StatusCode::NOT_FOUND,
                message: POST_NOT_FOUND_ERROR.to_string(),
            });
        }
    };

    let tags = post.find_related(Tags).all(&app_state.db).await?;
    Ok(Json(PostResponse::new(post, tags)))
}

pub async fn create_post(
    Extension(user_info): Extension<UserInfo>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ModifyPostRequest>,
) -> Result<Json<Post>, AppError> {
    let post = create_new_post(&user_info, &app_state, &payload).await?;
    let tags = insert_tags(&app_state, &payload).await?;
    associate_tags_with_post(&app_state, &post, &tags).await?;

    Ok(Json(post))
}

async fn create_new_post(
    user_info: &UserInfo,
    app_state: &Arc<AppState>,
    payload: &ModifyPostRequest,
) -> Result<Post, AppError> {
    let post = PostActiveModel {
        author: ActiveValue::Set(Uuid::from_str(&user_info.id).unwrap()),
        title: ActiveValue::Set(payload.title.clone()),
        content: ActiveValue::Set(payload.content.clone()),
        summary: ActiveValue::Set(payload.summary.clone()),
        category: ActiveValue::Set(Uuid::from_str(&payload.category).unwrap()),
        slug: ActiveValue::Set(payload.slug.clone()),
        read_time_millis: ActiveValue::Set(payload.read_time_millis),
        cover_image: ActiveValue::Set(payload.cover_image.clone()),
        ..Default::default()
    };

    post.insert(&app_state.db).await.map_err(|err| err.into())
}

pub async fn update_post(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ModifyPostRequest>,
) -> Result<Json<Post>, AppError> {
    let post = Posts::find_by_id(id).one(&app_state.db).await?;

    let mut post: PostActiveModel = match post {
        Some(post) => post.into(),
        None => {
            return Err(AppError {
                status: StatusCode::NOT_FOUND,
                message: POST_NOT_FOUND_ERROR.to_string(),
            });
        }
    };

    post.title = ActiveValue::Set(payload.title.clone());
    post.content = ActiveValue::Set(payload.content.clone());
    post.summary = ActiveValue::Set(payload.summary.clone());
    post.category = ActiveValue::Set(Uuid::from_str(&payload.category).unwrap());
    post.read_time_millis = ActiveValue::Set(payload.read_time_millis);
    post.cover_image = ActiveValue::Set(payload.cover_image.clone());

    let updates_post = post.update(&app_state.db).await?;

    // remove and add tags
    let tags = insert_tags(&app_state, &payload).await?;
    associate_tags_with_post(&app_state, &updates_post, &tags).await?;

    Ok(Json(updates_post))
}

async fn insert_tags(
    app_state: &Arc<AppState>,
    payload: &ModifyPostRequest,
) -> Result<Vec<Tag>, AppError> {
    let tag_models = payload.tags.iter().map(|tag| TagActiveModel {
        name: ActiveValue::Set(tag.to_string()),
        ..Default::default()
    });

    Tags::insert_many(tag_models)
        .on_conflict(OnConflict::column(TagColumn::Name).do_nothing().to_owned())
        .do_nothing()
        .exec(&app_state.db)
        .await?;

    let tags = Tags::find()
        .filter(TagColumn::Name.is_in(payload.tags.clone()))
        .all(&app_state.db)
        .await?;

    Ok(tags)
}

async fn associate_tags_with_post(
    app_state: &Arc<AppState>,
    post: &Post,
    tags: &[Tag],
) -> Result<(), AppError> {
    let post_tags = tags
        .iter()
        .map(|tag| PostTagActiveModel {
            post_id: ActiveValue::Set(post.id),
            tag_id: ActiveValue::Set(tag.id),
            ..Default::default()
        })
        .collect::<Vec<PostTagActiveModel>>();

    PostTags::delete_many()
        .filter(
            Condition::all().add(PostTagColumn::PostId.eq(post.id)).add(
                PostTagColumn::TagId
                    .is_not_in(tags.iter().map(|tag| tag.id as Uuid).collect::<Vec<Uuid>>()),
            ),
        )
        .exec(&app_state.db)
        .await?;

    PostTags::insert_many(post_tags)
        .on_conflict(
            OnConflict::columns([PostTagColumn::PostId, PostTagColumn::TagId])
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(&app_state.db)
        .await?;

    Ok(())
}
