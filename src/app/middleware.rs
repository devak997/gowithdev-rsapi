use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use super::{
    auth::model::{Claims, UserInfo},
    common::{core::AppState, errors::AppError},
};

const TOKEN_COOKIE_NAME: &str = "token";
const UNAUTHORIZED_ERROR: &str = "Unauthorized";

pub async fn authenticate(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = jar
        .get(TOKEN_COOKIE_NAME)
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, UNAUTHORIZED_ERROR.to_string()))?
        .value()
        .to_string();

    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(app_state.config.jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, UNAUTHORIZED_ERROR.to_string()))?;

    req.extensions_mut().insert(UserInfo {
        id: token_data.claims.sub,
    });

    let response = next.run(req).await;
    Ok(response)
}
