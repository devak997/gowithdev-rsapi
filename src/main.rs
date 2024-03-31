mod app;

use s3::{creds::Credentials, Region};
use std::net::SocketAddr;
use std::sync::Arc;

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use tracing::Level;
use tracing_subscriber::fmt::Subscriber;

use crate::app::{
    common::core::{AppState, Config},
    router::create_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = Subscriber::builder().with_max_level(Level::DEBUG).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let _ = dotenv();

    let config = Config::init()?;
    let db: DatabaseConnection = Database::connect(&config.database_url).await?;

    let creds = Credentials::new(
        Some(&config.b2_api_key_id),
        Some(&config.b2_api_secret),
        None,
        None,
        Some("b2_creds"),
    )?;

    let region = Region::Custom { region: config.b2_region.to_string(), endpoint: config.b2_endpoint.to_string() };

    let public_bucket = s3::bucket::Bucket::new(
        &config.b2_public_bucket,
        region,
        creds,
    )?;

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let router = create_router(Arc::new(AppState {
        db,
        config,
        public_bucket,
    }));

    println!("Running on http://{}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}
