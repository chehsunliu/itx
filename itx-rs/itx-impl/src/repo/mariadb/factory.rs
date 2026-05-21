use crate::repo::mariadb::post::MariaDbPostRepo;
use crate::repo::mariadb::subscription::MariaDbSubscriptionRepo;
use crate::repo::mariadb::user::MariaDbUserRepo;
use itx_contract::repo::factory::RepoFactory;
use itx_contract::repo::post::PostRepo;
use itx_contract::repo::subscription::SubscriptionRepo;
use itx_contract::repo::user::UserRepo;
use sqlx::MySqlPool;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode};
use std::sync::Arc;

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MariaDbRepoFactoryProps {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    pub password: String,
}

pub struct MariaDbRepoFactory {
    pool: MySqlPool,
}

impl MariaDbRepoFactory {
    pub async fn new(props: MariaDbRepoFactoryProps) -> Result<Self, sqlx::Error> {
        let options = MySqlConnectOptions::new()
            .host(&props.host)
            .port(props.port)
            .database(&props.db_name)
            .username(&props.user)
            .password(&props.password)
            .ssl_mode(MySqlSslMode::Disabled);

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;
        Ok(Self { pool })
    }
}

impl RepoFactory for MariaDbRepoFactory {
    fn create_post_repo(&self) -> Arc<dyn PostRepo> {
        Arc::new(MariaDbPostRepo::new(self.pool.clone()))
    }

    fn create_user_repo(&self) -> Arc<dyn UserRepo> {
        Arc::new(MariaDbUserRepo::new(self.pool.clone()))
    }

    fn create_subscription_repo(&self) -> Arc<dyn SubscriptionRepo> {
        Arc::new(MariaDbSubscriptionRepo::new(self.pool.clone()))
    }
}
