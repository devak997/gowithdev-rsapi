use std::sync::Arc;

use axum::{routing::get, Router};

use crate::api::controllers;
use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    return Router::new()
        .route("/api/posts", get(controllers::posts::get_posts()))
        .route("/api/posts/:id", get(controllers::posts::get_post()))
        .with_state(app_state);
}
