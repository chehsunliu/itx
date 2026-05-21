package sqs

import (
	"context"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/sqs"
	"github.com/chehsunliu/itx/itx-go/itx-contract/queue"
)

type FactoryProps struct {
	LocalEndpointURL        string `yaml:"local-endpoint-url"`
	MaxConcurrency          int64  `yaml:"max-concurrency"`
	ControlStandardQueueURL string `yaml:"control-standard-queue-url"`
	ControlPremiumQueueURL  string `yaml:"control-premium-queue-url"`
	ComputeStandardQueueURL string `yaml:"compute-standard-queue-url"`
	ComputePremiumQueueURL  string `yaml:"compute-premium-queue-url"`
}

type MessageQueueFactory struct {
	client *sqs.Client
	props  FactoryProps
}

func New(props FactoryProps) (*MessageQueueFactory, error) {
	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		return nil, err
	}

	client := sqs.NewFromConfig(cfg, func(o *sqs.Options) {
		if props.LocalEndpointURL != "" {
			o.BaseEndpoint = &props.LocalEndpointURL
		}
	})
	return &MessageQueueFactory{client: client, props: props}, nil
}

func (f *MessageQueueFactory) CreateControlStandardQueue() queue.MessageQueue {
	return newMessageQueue(f.client, f.props.ControlStandardQueueURL, f.props.MaxConcurrency)
}

func (f *MessageQueueFactory) CreateControlPremiumQueue() queue.MessageQueue {
	return newMessageQueue(f.client, f.props.ControlPremiumQueueURL, f.props.MaxConcurrency)
}

func (f *MessageQueueFactory) CreateComputeStandardQueue() queue.MessageQueue {
	return newMessageQueue(f.client, f.props.ComputeStandardQueueURL, f.props.MaxConcurrency)
}

func (f *MessageQueueFactory) CreateComputePremiumQueue() queue.MessageQueue {
	return newMessageQueue(f.client, f.props.ComputePremiumQueueURL, f.props.MaxConcurrency)
}
