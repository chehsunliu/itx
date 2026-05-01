use crate::error::BackendError;
use crate::feature::post::dto::PostDto;
use itx_contract::repo::post::{PostId, PostRepo, UpdateParams};
use std::sync::Arc;
use uuid::Uuid;

pub struct ExecuteParams {
    pub id: PostId,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub struct UpdatePostUseCase {
    post_repo: Arc<dyn PostRepo>,
}

impl UpdatePostUseCase {
    pub fn new(post_repo: Arc<dyn PostRepo>) -> Self {
        Self { post_repo }
    }

    pub async fn execute(&self, params: ExecuteParams) -> Result<PostDto, BackendError> {
        let post = self
            .post_repo
            .update(UpdateParams {
                id: params.id,
                author_id: params.user_id,
                title: params.title,
                body: params.body,
                tags: params.tags,
            })
            .await?;
        Ok(post.into())
    }
}
