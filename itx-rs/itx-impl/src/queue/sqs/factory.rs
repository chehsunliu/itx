use std::sync::Arc;

use aws_config::BehaviorVersion;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::config::{Credentials, Region};
use itx_contract::queue::MessageQueue;
use itx_contract::queue::factory::MessageQueueFactory;

use crate::queue::sqs::SqsMessageQueue;

#[derive(serde::Deserialize)]
struct SqsMessageQueueFactoryConfig {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint_url: Option<String>,
    pub control_standard_queue_url: String,
    pub control_premium_queue_url: String,
    pub compute_standard_queue_url: String,
    pub compute_premium_queue_url: String,
}

pub struct SqsMessageQueueFactory {
    pub client: Client,
    config: SqsMessageQueueFactoryConfig,
}

impl SqsMessageQueueFactory {
    pub async fn from_env() -> Self {
        let config = envy::prefixed("ITX_SQS_")
            .from_env::<SqsMessageQueueFactoryConfig>()
            .expect("failed to read SQS environment variables");

        let mut loader = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.region.clone()))
            .credentials_provider(Credentials::new(
                config.access_key_id.clone(),
                config.secret_access_key.clone(),
                None,
                None,
                "itx",
            ));
        if let Some(endpoint) = &config.endpoint_url {
            loader = loader.endpoint_url(endpoint.clone());
        }
        let client = Client::new(&loader.load().await);

        Self { client, config }
    }
}

impl MessageQueueFactory for SqsMessageQueueFactory {
    fn create_control_standard_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.config.control_standard_queue_url.clone(),
        ))
    }

    fn create_control_premium_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.config.control_premium_queue_url.clone(),
        ))
    }

    fn create_compute_standard_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.config.compute_standard_queue_url.clone(),
        ))
    }

    fn create_compute_premium_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.config.compute_premium_queue_url.clone(),
        ))
    }
}
