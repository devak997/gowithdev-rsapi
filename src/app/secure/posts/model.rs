use crate::app::orm::posts::Model as Post;
use crate::app::orm::tags::Model as Tag;
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};

#[derive(Debug, serde::Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub category: Uuid,
    pub slug: String,
    pub author: Uuid,
    pub cover_image: String,
    pub draft: Option<bool>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub read_time_millis: i64,
    pub tags: Vec<String>,
}

impl PostResponse {
    pub fn new(post: Post, tags: Vec<Tag>) -> Self {
        PostResponse {
            id: post.id,
            title: post.title,
            summary: post.summary,
            content: post.content,
            category: post.category,
            slug: post.slug,
            author: post.author,
            draft: post.draft,
            cover_image: post.cover_image,
            created_at: post.created_at,
            updated_at: post.updated_at,
            read_time_millis: post.read_time_millis,
            tags: tags.into_iter().map(|tag| tag.name).collect(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ModifyPostRequest {
    pub title: String,
    pub content: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub category: String,
    pub slug: String,
    pub read_time_millis: i64,
    pub cover_image: String,
}
