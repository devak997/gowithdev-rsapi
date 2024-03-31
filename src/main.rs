mod api;
mod config;
mod routes;

use std::sync::Arc;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

use config::Config;

use routes::create_router;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();
    let db: DatabaseConnection = Database::connect(&config.database_url).await.unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = create_router(Arc::new(AppState {
        db: db.clone(),
        config: config.clone(),
    }));
    println!("Running on http://localhost:3000");
    axum::serve(listener, router).await.unwrap();
}
