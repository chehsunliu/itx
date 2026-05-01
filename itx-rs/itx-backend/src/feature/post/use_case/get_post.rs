use crate::error::BackendError;
use crate::feature::post::dto::PostDto;
use itx_contract::repo::post::{GetParams, PostId, PostRepo};
use std::sync::Arc;
use uuid::Uuid;

pub struct ExecuteParams {
    pub id: PostId,
    pub user_id: Uuid,
}

pub struct GetPostUseCase {
    post_repo: Arc<dyn PostRepo>,
}

impl GetPostUseCase {
    pub fn new(post_repo: Arc<dyn PostRepo>) -> Self {
        Self { post_repo }
    }

    pub async fn execute(&self, params: ExecuteParams) -> Result<PostDto, BackendError> {
        let post = self.post_repo.get(GetParams { id: params.id }).await?;
        if post.author_id != params.user_id {
            return Err(BackendError::NotFound);
        }
        Ok(post.into())
    }
}
