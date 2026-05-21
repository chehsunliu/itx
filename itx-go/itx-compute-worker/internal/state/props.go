package state

import (
	_ "embed"
	"fmt"

	"github.com/chehsunliu/itx/itx-go/itx-impl/config"
	"github.com/chehsunliu/itx/itx-go/itx-impl/queue/rabbitmq"
	"github.com/chehsunliu/itx/itx-go/itx-impl/queue/sqs"
	"gopkg.in/yaml.v3"
)

//go:embed application.yaml
var rawApplicationYAML string

type WorkerProps struct {
	QueueProvider string                `yaml:"queue-provider"`
	RabbitMQ      rabbitmq.FactoryProps `yaml:"rabbitmq"`
	SQS           sqs.FactoryProps      `yaml:"sqs"`
}

func PropsFromEnv() (WorkerProps, error) {
	var props WorkerProps
	if err := yaml.Unmarshal([]byte(config.SubstituteEnv(rawApplicationYAML)), &props); err != nil {
		return WorkerProps{}, fmt.Errorf("parse application.yaml: %w", err)
	}
	return props, nil
}
