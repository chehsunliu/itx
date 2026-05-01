use crate::error::BackendError;
use crate::feature::post::dto::PostDto;
use itx_contract::repo::post::{CreateParams, PostRepo};
use std::sync::Arc;
use uuid::Uuid;

pub struct ExecuteParams {
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

pub struct CreatePostUseCase {
    post_repo: Arc<dyn PostRepo>,
}

impl CreatePostUseCase {
    pub fn new(post_repo: Arc<dyn PostRepo>) -> Self {
        Self { post_repo }
    }

    pub async fn execute(&self, params: ExecuteParams) -> Result<PostDto, BackendError> {
        let post = self
            .post_repo
            .create(CreateParams {
                author_id: params.user_id,
                title: params.title,
                body: params.body,
                tags: params.tags,
            })
            .await?;
        Ok(post.into())
    }
}
