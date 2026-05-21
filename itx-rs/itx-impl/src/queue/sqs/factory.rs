use std::sync::Arc;

use aws_sdk_sqs::Client;
use itx_contract::queue::MessageQueue;
use itx_contract::queue::factory::MessageQueueFactory;

use crate::queue::sqs::SqsMessageQueue;

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SqsMessageQueueFactoryProps {
    pub local_endpoint_url: Option<String>,
    pub max_concurrency: u32,
    pub control_standard_queue_url: String,
    pub control_premium_queue_url: String,
    pub compute_standard_queue_url: String,
    pub compute_premium_queue_url: String,
}

pub struct SqsMessageQueueFactory {
    client: Client,
    props: SqsMessageQueueFactoryProps,
}

impl SqsMessageQueueFactory {
    pub async fn new(props: SqsMessageQueueFactoryProps) -> Self {
        let aws_config = aws_config::load_from_env().await;
        let mut sqs_config = aws_sdk_sqs::config::Builder::from(&aws_config);
        if let Some(endpoint) = &props.local_endpoint_url {
            sqs_config = sqs_config.endpoint_url(endpoint);
        }
        let client = Client::from_conf(sqs_config.build());

        Self { client, props }
    }
}

impl MessageQueueFactory for SqsMessageQueueFactory {
    fn create_control_standard_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.props.control_standard_queue_url.clone(),
            self.props.max_concurrency,
        ))
    }

    fn create_control_premium_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.props.control_premium_queue_url.clone(),
            self.props.max_concurrency,
        ))
    }

    fn create_compute_standard_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.props.compute_standard_queue_url.clone(),
            self.props.max_concurrency,
        ))
    }

    fn create_compute_premium_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(SqsMessageQueue::new(
            self.client.clone(),
            self.props.compute_premium_queue_url.clone(),
            self.props.max_concurrency,
        ))
    }
}
