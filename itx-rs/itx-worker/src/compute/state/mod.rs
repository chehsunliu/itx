mod props;

use std::error::Error;
use std::sync::Arc;

use crate::compute::state::props::ComputeWorkerProps;
use itx_contract::queue::MessageQueue;
use itx_contract::queue::factory::MessageQueueFactory;
use itx_impl::queue::rabbitmq::factory::RabbitMessageQueueFactory;
use itx_impl::queue::sqs::factory::SqsMessageQueueFactory;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ComputeWorkerStateProps {
    pub queue_provider: Option<String>,
}

#[derive(Clone)]
pub struct ComputeWorkerState {
    pub props: ComputeWorkerProps,
    pub control_standard_queue: Arc<dyn MessageQueue>,
    pub control_premium_queue: Arc<dyn MessageQueue>,
    pub compute_standard_queue: Arc<dyn MessageQueue>,
    pub compute_premium_queue: Arc<dyn MessageQueue>,
}

impl ComputeWorkerState {
    pub async fn from_env() -> Result<Self, Box<dyn Error>> {
        let props = ComputeWorkerProps::from_env()?;
        let queue_factory: Arc<dyn MessageQueueFactory> = match props.queue_provider.as_deref().unwrap_or("sqs") {
            "sqs" => Arc::new(SqsMessageQueueFactory::new(props.sqs.clone()).await),
            "rabbitmq" => Arc::new(RabbitMessageQueueFactory::new(props.rabbitmq.clone()).await?),
            other => panic!("unknown ITX_QUEUE_PROVIDER: {other}"),
        };

        Ok(Self {
            props,
            control_standard_queue: queue_factory.create_control_standard_queue(),
            control_premium_queue: queue_factory.create_control_premium_queue(),
            compute_standard_queue: queue_factory.create_compute_standard_queue(),
            compute_premium_queue: queue_factory.create_compute_premium_queue(),
        })
    }
}
