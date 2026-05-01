use crate::error::BackendError;
use crate::feature::post::use_case::list_posts;
use crate::middleware::context::ItxContext;
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Extension, Json, Router};
use itx_contract::repo::post::PostRepo;
use serde::Deserialize;
use std::sync::Arc;

pub mod use_case;

#[derive(Deserialize)]
struct ListPostsQuery {
    #[serde(default = "default_limit")]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

fn default_limit() -> u32 {
    50
}

async fn list_posts(
    State(post_repo): State<Arc<dyn PostRepo>>,
    Extension(context): Extension<ItxContext>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<list_posts::ExecuteOutput>, BackendError> {
    let params = list_posts::ExecuteParams {
        user_id: context.user_id.unwrap(),
        limit: query.limit,
        offset: query.offset,
    };
    let use_case = list_posts::ListPostsUseCase::new(post_repo);
    Ok(Json(use_case.execute(params).await?))
}

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", get(list_posts))
}
