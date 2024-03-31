use axum::response::IntoResponse;
use sea_orm::DbErr;

pub struct AppError {
    pub status: axum::http::StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(status: axum::http::StatusCode, message: String) -> Self {
        Self { status, message }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        }
    }
}
