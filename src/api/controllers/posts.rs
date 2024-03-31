use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::AppState;

pub fn get_posts(State(app_state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    return Ok("Hello".to_string());
}

pub fn get_post() {
    // Get One Particular Post
}
