use std::sync::Arc;

use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::response::{AppendHeaders, IntoResponse};
use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie, Expiration, SameSite};

use crate::app::auth::model::Claims;
use crate::app::common::core::AppState;
use crate::app::common::errors::AppError;
use crate::app::orm::users::{Column, Entity as Users, Model as User};
use sea_orm::{entity::*, query::*};

use super::model::LoginRequestPayload;

const INVALID_CREDENTIALS_ERROR: &str = "Invalid email or password";

pub async fn verify_login(
    State(app_state): State<Arc<AppState>>,
    payload_r: Result<Json<LoginRequestPayload>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let payload = payload_r.map_err(|_| AppError {
        message: "Invalid payload".to_string(),
        status: StatusCode::BAD_REQUEST,
    })?;

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError {
            message: "Email and password are required".to_string(),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let user = Users::find()
        .filter(Column::Email.eq(&payload.email))
        .columns([Column::Email, Column::PasswordHash, Column::Id])
        .one(&app_state.db)
        .await
        .map_err(|err| AppError {
            message: err.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError {
                message: INVALID_CREDENTIALS_ERROR.to_string(),
                status: StatusCode::UNAUTHORIZED,
            })
        }
    };

    let matched = bcrypt::verify(&payload.password, &user.password_hash).map_err(|_| AppError {
        message: INVALID_CREDENTIALS_ERROR.to_string(),
        status: StatusCode::UNAUTHORIZED,
    })?;

    if !matched {
        return Err(AppError {
            message: INVALID_CREDENTIALS_ERROR.to_string(),
            status: StatusCode::UNAUTHORIZED,
        });
    }

    let token = generate_token(&user, &app_state)?;

    let expire_time =
        Expiration::DateTime(time::OffsetDateTime::now_utc() + time::Duration::hours(1));
    let headers = AppendHeaders([(
        SET_COOKIE,
        Cookie::build(("token", token))
            .path("/")
            .secure(true)
            .http_only(true)
            .expires(expire_time)
            .same_site(SameSite::Strict)
            .to_string(),
    )]);

    Ok((headers, ()))
}

fn generate_token(user: &User, app_state: &AppState) -> Result<String, AppError> {
    let expire_time = chrono::Utc::now() + chrono::Duration::hours(1);
    let exp: u64 = (expire_time).timestamp().unsigned_abs();

    let claims = Claims {
        sub: user.id.to_string(),
        exp: exp as usize,
        iat: chrono::Utc::now().timestamp().unsigned_abs() as usize,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(app_state.config.jwt_secret.as_ref()),
    );

    token.map_err(|_| AppError {
        message: "Failed to generate token".to_string(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })
}
