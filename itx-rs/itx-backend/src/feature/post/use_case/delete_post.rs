use crate::error::BackendError;
use itx_contract::repo::post::{DeleteParams, PostId, PostRepo};
use std::sync::Arc;
use uuid::Uuid;

pub struct ExecuteParams {
    pub id: PostId,
    pub user_id: Uuid,
}

pub struct DeletePostUseCase {
    post_repo: Arc<dyn PostRepo>,
}

impl DeletePostUseCase {
    pub fn new(post_repo: Arc<dyn PostRepo>) -> Self {
        Self { post_repo }
    }

    pub async fn execute(&self, params: ExecuteParams) -> Result<(), BackendError> {
        self.post_repo
            .delete(DeleteParams {
                id: params.id,
                author_id: params.user_id,
            })
            .await?;
        Ok(())
    }
}
