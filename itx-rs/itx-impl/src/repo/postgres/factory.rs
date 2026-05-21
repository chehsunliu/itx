use crate::repo::postgres::post::PostgresPostRepo;
use crate::repo::postgres::subscription::PostgresSubscriptionRepo;
use crate::repo::postgres::user::PostgresUserRepo;
use itx_contract::repo::factory::RepoFactory;
use itx_contract::repo::post::PostRepo;
use itx_contract::repo::subscription::SubscriptionRepo;
use itx_contract::repo::user::UserRepo;
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::sync::Arc;

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostgresRepoFactoryProps {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    pub password: String,
}

pub struct PostgresRepoFactory {
    pool: PgPool,
}

impl PostgresRepoFactory {
    pub async fn new(props: PostgresRepoFactoryProps) -> Result<Self, sqlx::Error> {
        let options = PgConnectOptions::new()
            .host(&props.host)
            .port(props.port)
            .database(&props.db_name)
            .username(&props.user)
            .password(&props.password);

        let pool = PgPoolOptions::new().max_connections(10).connect_with(options).await?;
        Ok(Self { pool })
    }
}

impl RepoFactory for PostgresRepoFactory {
    fn create_post_repo(&self) -> Arc<dyn PostRepo> {
        Arc::new(PostgresPostRepo::new(self.pool.clone()))
    }

    fn create_user_repo(&self) -> Arc<dyn UserRepo> {
        Arc::new(PostgresUserRepo::new(self.pool.clone()))
    }

    fn create_subscription_repo(&self) -> Arc<dyn SubscriptionRepo> {
        Arc::new(PostgresSubscriptionRepo::new(self.pool.clone()))
    }
}
