use std::sync::Arc;
use std::time::Duration;

use axum::extract::{MatchedPath, Request};
use axum::http::StatusCode;
use axum::middleware::{self};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

use crate::AppState;

use super::middleware::authenticate;
use super::{auth, public, secure};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        })
        .on_request(|_request: &Request<_>, _span: &Span| {
            tracing::info!(
                "request received | {method} | {uri}",
                method = _request.method(),
                uri = _request.uri()
            );
        })
        .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
            tracing::info!(
                "response sent | {status} | {latency:?}",
                status = _response.status(),
                latency = _latency
            );
        });

    return Router::new()
        .route("/admin/posts", post(secure::posts::controller::create_post))
        .route(
            "/admin/posts/:id",
            get(secure::posts::controller::get_post).post(secure::posts::controller::update_post),
        )
        .route(
            "/admin/categories",
            get(secure::categories::controller::get_categories),
        )
        .route("/me", get(secure::user::controller::get_current_user_info))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            authenticate,
        ))
        .route("/login", post(auth::login::controller::verify_login))
        .route("/posts", get(public::posts::controller::get_posts))
        .route("/posts/:id", get(public::posts::controller::get_post))
        .route(
            "/media/pre-signed-url",
            get(secure::media::controller::get_presigned_url),
        )
        .fallback(handler_404)
        .layer(trace_layer)
        .with_state(app_state);
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
