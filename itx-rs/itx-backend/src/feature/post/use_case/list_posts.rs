use crate::error::BackendError;
use crate::feature::post::dto::PostDto;
use itx_contract::repo::post::{ListParams, PostRepo};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

pub struct ExecuteParams {
    pub user_id: Uuid,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOutput {
    pub items: Vec<PostDto>,
}

pub struct ListPostsUseCase {
    post_repo: Arc<dyn PostRepo>,
}

impl ListPostsUseCase {
    pub fn new(post_repo: Arc<dyn PostRepo>) -> Self {
        Self { post_repo }
    }

    pub async fn execute(&self, params: ExecuteParams) -> Result<ExecuteOutput, BackendError> {
        let posts = self
            .post_repo
            .list(ListParams {
                author_id: Some(params.user_id),
                limit: params.limit,
                offset: params.offset,
            })
            .await?;

        Ok(ExecuteOutput {
            items: posts.into_iter().map(PostDto::from).collect(),
        })
    }
}
