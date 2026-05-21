use itx_impl::queue::rabbitmq::factory::RabbitMessageQueueFactoryProps;
use itx_impl::queue::sqs::factory::SqsMessageQueueFactoryProps;
use itx_impl::repo::mariadb::factory::MariaDbRepoFactoryProps;
use itx_impl::repo::postgres::factory::PostgresRepoFactoryProps;
use serde::Deserialize;
use std::error::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppProps {
    pub db_provider: Option<String>,
    pub queue_provider: Option<String>,
    pub mariadb: MariaDbRepoFactoryProps,
    pub postgres: PostgresRepoFactoryProps,
    pub rabbitmq: RabbitMessageQueueFactoryProps,
    pub sqs: SqsMessageQueueFactoryProps,
}

impl AppProps {
    pub fn from_env() -> Result<AppProps, Box<dyn Error>> {
        let raw_props_content = include_str!("./application.yaml");
        let props_content = subst::substitute(raw_props_content, &subst::Env)?;
        let props = serde_yaml::from_str::<AppProps>(&props_content)?;
        Ok(props)
    }
}
