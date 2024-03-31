use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
    pub public_bucket: Box<s3::bucket::Bucket>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
    pub b2_api_key_id: String,
    pub b2_api_secret: String,
    pub b2_endpoint: String,
    pub b2_public_bucket: String,
    pub b2_region: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Result<Config, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let b2_api_key_id = std::env::var("B2_API_KEY_ID")?;
        let b2_api_secret = std::env::var("B2_API_KEY")?;
        let b2_endpoint = std::env::var("B2_ENDPOINT")?;
        let b2_public_bucket = std::env::var("B2_PUBLIC_BUCKET")?;
        let b2_region = std::env::var("B2_REGION")?;

        let host = std::env::var("HOST")?;
        let port = std::env::var("PORT")?.parse::<u16>()?;

        Ok(Config {
            database_url,
            jwt_secret,
            b2_api_key_id,
            b2_api_secret,
            b2_endpoint,
            b2_public_bucket,
            b2_region,
            host,
            port,
        })
    }
}
