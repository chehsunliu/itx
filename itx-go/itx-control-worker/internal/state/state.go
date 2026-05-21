package state

import (
	"fmt"

	"github.com/chehsunliu/itx/itx-go/itx-contract/email"
	contractqueue "github.com/chehsunliu/itx/itx-go/itx-contract/queue"
	queuefactory "github.com/chehsunliu/itx/itx-go/itx-contract/queue/factory"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/factory"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/post"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/subscription"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/user"
	implemail "github.com/chehsunliu/itx/itx-go/itx-impl/email"
	queuerabbit "github.com/chehsunliu/itx/itx-go/itx-impl/queue/rabbitmq"
	queuesqs "github.com/chehsunliu/itx/itx-go/itx-impl/queue/sqs"
	"github.com/chehsunliu/itx/itx-go/itx-impl/repo/mariadb"
	"github.com/chehsunliu/itx/itx-go/itx-impl/repo/postgres"
)

type WorkerState struct {
	Props                WorkerProps
	PostRepo             post.Repo
	UserRepo             user.Repo
	SubscriptionRepo     subscription.Repo
	ControlStandardQueue contractqueue.MessageQueue
	ControlPremiumQueue  contractqueue.MessageQueue
	ComputeStandardQueue contractqueue.MessageQueue
	ComputePremiumQueue  contractqueue.MessageQueue
	EmailClient          email.EmailClient
}

func FromEnv() (WorkerState, error) {
	props, err := PropsFromEnv()
	if err != nil {
		return WorkerState{}, err
	}

	dbProvider := props.DBProvider
	if dbProvider == "" {
		dbProvider = "postgres"
	}
	var repoFactory factory.RepoFactory
	switch dbProvider {
	case "postgres":
		f, err := postgres.New(props.Postgres)
		if err != nil {
			return WorkerState{}, err
		}
		repoFactory = f
	case "mariadb":
		f, err := mariadb.New(props.MariaDB)
		if err != nil {
			return WorkerState{}, err
		}
		repoFactory = f
	default:
		return WorkerState{}, fmt.Errorf("unknown ITX_DB_PROVIDER: %s", dbProvider)
	}

	queueProvider := props.QueueProvider
	if queueProvider == "" {
		queueProvider = "sqs"
	}
	var queueFactory queuefactory.MessageQueueFactory
	switch queueProvider {
	case "sqs":
		f, err := queuesqs.New(props.SQS)
		if err != nil {
			return WorkerState{}, err
		}
		queueFactory = f
	case "rabbitmq":
		f, err := queuerabbit.New(props.RabbitMQ)
		if err != nil {
			return WorkerState{}, err
		}
		queueFactory = f
	default:
		return WorkerState{}, fmt.Errorf("unknown ITX_QUEUE_PROVIDER: %s", queueProvider)
	}

	return WorkerState{
		Props:                props,
		PostRepo:             repoFactory.CreatePostRepo(),
		UserRepo:             repoFactory.CreateUserRepo(),
		SubscriptionRepo:     repoFactory.CreateSubscriptionRepo(),
		ControlStandardQueue: queueFactory.CreateControlStandardQueue(),
		ControlPremiumQueue:  queueFactory.CreateControlPremiumQueue(),
		ComputeStandardQueue: queueFactory.CreateComputeStandardQueue(),
		ComputePremiumQueue:  queueFactory.CreateComputePremiumQueue(),
		EmailClient:          implemail.FromEnv(),
	}, nil
}
