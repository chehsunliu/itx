package rabbitmq

import (
	"fmt"

	"github.com/chehsunliu/itx/itx-go/itx-contract/queue"
	amqp "github.com/rabbitmq/amqp091-go"
)

type FactoryProps struct {
	Host                 string `yaml:"host"`
	Port                 int    `yaml:"port"`
	User                 string `yaml:"user"`
	Password             string `yaml:"password"`
	MaxConcurrency       int64  `yaml:"max-concurrency"`
	ControlStandardQueue string `yaml:"control-standard-queue"`
	ControlPremiumQueue  string `yaml:"control-premium-queue"`
	ComputeStandardQueue string `yaml:"compute-standard-queue"`
	ComputePremiumQueue  string `yaml:"compute-premium-queue"`
}

type MessageQueueFactory struct {
	conn  *amqp.Connection
	props FactoryProps
}

func New(props FactoryProps) (*MessageQueueFactory, error) {
	url := fmt.Sprintf("amqp://%s:%s@%s:%d/%%2F", props.User, props.Password, props.Host, props.Port)
	conn, err := amqp.Dial(url)
	if err != nil {
		return nil, err
	}
	return &MessageQueueFactory{conn: conn, props: props}, nil
}

func (f *MessageQueueFactory) CreateControlStandardQueue() queue.MessageQueue {
	return newMessageQueue(f.conn, f.props.ControlStandardQueue, f.props.MaxConcurrency)
}

func (f *MessageQueueFactory) CreateControlPremiumQueue() queue.MessageQueue {
	return newMessageQueue(f.conn, f.props.ControlPremiumQueue, f.props.MaxConcurrency)
}

func (f *MessageQueueFactory) CreateComputeStandardQueue() queue.MessageQueue {
	return newMessageQueue(f.conn, f.props.ComputeStandardQueue, f.props.MaxConcurrency)
}

func (f *MessageQueueFactory) CreateComputePremiumQueue() queue.MessageQueue {
	return newMessageQueue(f.conn, f.props.ComputePremiumQueue, f.props.MaxConcurrency)
}
