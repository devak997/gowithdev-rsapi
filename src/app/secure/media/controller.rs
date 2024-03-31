use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use uuid::Uuid;

use crate::app::common::{core::AppState, errors::AppError};

use super::model::PreSignedUrlResponse;

fn map_to_app_error(err: impl ToString) -> AppError {
    AppError {
        message: err.to_string(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_presigned_url(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<PreSignedUrlResponse>, AppError> {
    let path = format!("/media/{}", Uuid::new_v4());

    let presigned_uri = app_state
        .public_bucket
        .presign_put(&path, 60, None, None)
        .await
        .map_err(map_to_app_error)?;

    Ok(Json(PreSignedUrlResponse {
        url: presigned_uri,
        path,
    }))
}
