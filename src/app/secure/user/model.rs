use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CurrentUserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}
