use std::sync::Arc;

use itx_contract::queue::MessageQueue;
use itx_contract::queue::factory::MessageQueueFactory;
use lapin::{Connection, ConnectionProperties};

use crate::queue::rabbitmq::RabbitMessageQueue;

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RabbitMessageQueueFactoryProps {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub max_concurrency: u32,
    pub control_standard_queue: String,
    pub control_premium_queue: String,
    pub compute_standard_queue: String,
    pub compute_premium_queue: String,
}

pub struct RabbitMessageQueueFactory {
    conn: Arc<Connection>,
    props: RabbitMessageQueueFactoryProps,
}

impl RabbitMessageQueueFactory {
    pub async fn new(props: RabbitMessageQueueFactoryProps) -> Result<Self, lapin::Error> {
        let url = format!(
            "amqp://{}:{}@{}:{}/%2F",
            props.user, props.password, props.host, props.port
        );
        let conn = Connection::connect(&url, ConnectionProperties::default()).await?;

        Ok(Self {
            conn: Arc::new(conn),
            props,
        })
    }
}

impl MessageQueueFactory for RabbitMessageQueueFactory {
    fn create_control_standard_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(RabbitMessageQueue::new(
            self.conn.clone(),
            self.props.control_standard_queue.clone(),
            self.props.max_concurrency,
        ))
    }

    fn create_control_premium_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(RabbitMessageQueue::new(
            self.conn.clone(),
            self.props.control_premium_queue.clone(),
            self.props.max_concurrency,
        ))
    }

    fn create_compute_standard_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(RabbitMessageQueue::new(
            self.conn.clone(),
            self.props.compute_standard_queue.clone(),
            self.props.max_concurrency,
        ))
    }

    fn create_compute_premium_queue(&self) -> Arc<dyn MessageQueue> {
        Arc::new(RabbitMessageQueue::new(
            self.conn.clone(),
            self.props.compute_premium_queue.clone(),
            self.props.max_concurrency,
        ))
    }
}
