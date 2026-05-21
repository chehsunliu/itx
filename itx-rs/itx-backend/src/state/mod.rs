pub mod props;

use crate::state::props::AppProps;
use axum::extract::FromRef;
use itx_contract::queue::MessageQueue;
use itx_contract::queue::factory::MessageQueueFactory;
use itx_contract::repo::factory::RepoFactory;
use itx_contract::repo::post::PostRepo;
use itx_contract::repo::subscription::SubscriptionRepo;
use itx_contract::repo::user::UserRepo;
use itx_impl::queue::rabbitmq::factory::RabbitMessageQueueFactory;
use itx_impl::queue::sqs::factory::SqsMessageQueueFactory;
use itx_impl::repo::mariadb::factory::MariaDbRepoFactory;
use itx_impl::repo::postgres::factory::PostgresRepoFactory;
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub props: AppProps,
    pub post_repo: Arc<dyn PostRepo>,
    pub user_repo: Arc<dyn UserRepo>,
    pub subscription_repo: Arc<dyn SubscriptionRepo>,
    pub control_standard_queue: Arc<dyn MessageQueue>,
}

impl AppState {
    pub async fn from_env() -> Result<Self, Box<dyn Error>> {
        let props = AppProps::from_env().expect("failed to read app props environment variables");
        let repo_factory: Arc<dyn RepoFactory> = match props.db_provider.as_deref().unwrap_or("postgres") {
            "postgres" => Arc::new(PostgresRepoFactory::new(props.postgres.clone()).await?),
            "mariadb" => Arc::new(MariaDbRepoFactory::new(props.mariadb.clone()).await?),
            other => panic!("unknown ITX_DB_PROVIDER: {other}"),
        };
        let queue_factory: Arc<dyn MessageQueueFactory> = match props.queue_provider.as_deref().unwrap_or("sqs") {
            "sqs" => Arc::new(SqsMessageQueueFactory::new(props.sqs.clone()).await),
            "rabbitmq" => Arc::new(RabbitMessageQueueFactory::new(props.rabbitmq.clone()).await?),
            other => panic!("unknown ITX_QUEUE_PROVIDER: {other}"),
        };

        Ok(Self {
            props,
            post_repo: repo_factory.create_post_repo(),
            user_repo: repo_factory.create_user_repo(),
            subscription_repo: repo_factory.create_subscription_repo(),
            control_standard_queue: queue_factory.create_control_standard_queue(),
        })
    }
}

impl FromRef<AppState> for Arc<dyn PostRepo> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.post_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn UserRepo> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn SubscriptionRepo> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.subscription_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn MessageQueue> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.control_standard_queue.clone()
    }
}
