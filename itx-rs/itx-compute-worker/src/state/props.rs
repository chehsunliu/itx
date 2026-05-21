use itx_impl::queue::rabbitmq::factory::RabbitMessageQueueFactoryProps;
use itx_impl::queue::sqs::factory::SqsMessageQueueFactoryProps;
use serde::Deserialize;
use std::error::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ComputeWorkerProps {
    pub queue_provider: Option<String>,
    pub rabbitmq: RabbitMessageQueueFactoryProps,
    pub sqs: SqsMessageQueueFactoryProps,
}

impl ComputeWorkerProps {
    pub fn from_env() -> Result<ComputeWorkerProps, Box<dyn Error>> {
        let raw_props_content = include_str!("./application.yaml");
        let props_content = subst::substitute(raw_props_content, &subst::Env)?;
        let props = serde_yaml::from_str::<ComputeWorkerProps>(&props_content)?;
        Ok(props)
    }
}
