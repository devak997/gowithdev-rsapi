use std::str::FromStr;
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{EntityTrait, QuerySelect};
use uuid::Uuid;

use crate::app::orm::users::{Column, Entity as Users, Model as User};
use crate::app::{
    auth::model::UserInfo,
    common::{core::AppState, errors::AppError},
};

use super::model::CurrentUserResponse;

impl From<User> for CurrentUserResponse {
    fn from(user: User) -> Self {
        CurrentUserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
        }
    }
}

pub async fn get_current_user_info(
    State(app_state): State<Arc<AppState>>,
    Extension(user_info): Extension<UserInfo>,
) -> Result<Json<CurrentUserResponse>, AppError> {
    let user_id = user_info.id;

    let user = Users::find_by_id(Uuid::from_str(&user_id).unwrap())
        .columns([Column::Id, Column::Name, Column::Email, Column::Avatar])
        .one(&app_state.db)
        .await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError::new(
                StatusCode::NOT_FOUND,
                "User not found ".to_string(),
            ))
        }
    };

    Ok(Json(user.into()))
}
