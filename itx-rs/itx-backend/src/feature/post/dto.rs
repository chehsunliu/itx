use itx_contract::repo::post::Post;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDto {
    pub id: i64,
    pub author_id: Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
}

impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            title: post.title,
            body: post.body,
            tags: post.tags,
            created_at: post.created_at,
        }
    }
}
