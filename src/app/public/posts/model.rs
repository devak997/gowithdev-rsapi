use crate::app::orm::posts::Model as Post;
use crate::app::orm::tags::Model as Tag;
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PostResponse {
    id: Uuid,
    title: String,
    content: String,
    summary: String,
    updated_at: DateTimeWithTimeZone,
    tags: Vec<String>,
    cover_image: String,
    read_time_millis: i64,
}

impl PostResponse {
    pub fn new(post: &Post, tags: &Vec<Tag>) -> Self {
        PostResponse {
            id: post.id,
            title: post.title.clone(),
            content: post.content.clone(),
            summary: post.summary.clone(),
            updated_at: post.updated_at.unwrap(),
            tags: tags.into_iter().map(|tag| tag.name.clone()).collect(),
            cover_image: post.cover_image.clone(),
            read_time_millis: post.read_time_millis,
        }
    }
}
