#[derive(Debug, serde::Serialize)]
pub struct PreSignedUrlResponse {
    pub url: String,
    pub path: String,
}
