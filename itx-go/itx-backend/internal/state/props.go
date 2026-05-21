package state

import (
	_ "embed"
	"fmt"

	"github.com/chehsunliu/itx/itx-go/itx-impl/config"
	"github.com/chehsunliu/itx/itx-go/itx-impl/queue/rabbitmq"
	"github.com/chehsunliu/itx/itx-go/itx-impl/queue/sqs"
	"github.com/chehsunliu/itx/itx-go/itx-impl/repo/mariadb"
	"github.com/chehsunliu/itx/itx-go/itx-impl/repo/postgres"
	"gopkg.in/yaml.v3"
)

//go:embed application.yaml
var rawApplicationYAML string

type AppProps struct {
	DBProvider    string                    `yaml:"db-provider"`
	QueueProvider string                    `yaml:"queue-provider"`
	MariaDB       mariadb.RepoFactoryProps  `yaml:"mariadb"`
	Postgres      postgres.RepoFactoryProps `yaml:"postgres"`
	RabbitMQ      rabbitmq.FactoryProps     `yaml:"rabbitmq"`
	SQS           sqs.FactoryProps          `yaml:"sqs"`
}

func PropsFromEnv() (AppProps, error) {
	var props AppProps
	if err := yaml.Unmarshal([]byte(config.SubstituteEnv(rawApplicationYAML)), &props); err != nil {
		return AppProps{}, fmt.Errorf("parse application.yaml: %w", err)
	}
	return props, nil
}
