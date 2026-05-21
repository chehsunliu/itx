package state

import (
	"fmt"

	"github.com/chehsunliu/itx/itx-go/itx-contract/queue"
	queuefactory "github.com/chehsunliu/itx/itx-go/itx-contract/queue/factory"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/factory"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/post"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/subscription"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/user"
	queuerabbit "github.com/chehsunliu/itx/itx-go/itx-impl/queue/rabbitmq"
	queuesqs "github.com/chehsunliu/itx/itx-go/itx-impl/queue/sqs"
	"github.com/chehsunliu/itx/itx-go/itx-impl/repo/mariadb"
	"github.com/chehsunliu/itx/itx-go/itx-impl/repo/postgres"
)

type AppState struct {
	Props                AppProps
	PostRepo             post.Repo
	UserRepo             user.Repo
	SubscriptionRepo     subscription.Repo
	ControlStandardQueue queue.MessageQueue
}

func FromEnv() (AppState, error) {
	props, err := PropsFromEnv()
	if err != nil {
		return AppState{}, err
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
			return AppState{}, err
		}
		repoFactory = f
	case "mariadb":
		f, err := mariadb.New(props.MariaDB)
		if err != nil {
			return AppState{}, err
		}
		repoFactory = f
	default:
		return AppState{}, fmt.Errorf("unknown ITX_DB_PROVIDER: %s", dbProvider)
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
			return AppState{}, err
		}
		queueFactory = f
	case "rabbitmq":
		f, err := queuerabbit.New(props.RabbitMQ)
		if err != nil {
			return AppState{}, err
		}
		queueFactory = f
	default:
		return AppState{}, fmt.Errorf("unknown ITX_QUEUE_PROVIDER: %s", queueProvider)
	}

	return AppState{
		Props:                props,
		PostRepo:             repoFactory.CreatePostRepo(),
		UserRepo:             repoFactory.CreateUserRepo(),
		SubscriptionRepo:     repoFactory.CreateSubscriptionRepo(),
		ControlStandardQueue: queueFactory.CreateControlStandardQueue(),
	}, nil
}
