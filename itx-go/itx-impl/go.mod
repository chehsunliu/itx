module github.com/chehsunliu/itx/itx-go/itx-impl

go 1.25.4

require (
	github.com/aws/aws-sdk-go-v2 v1.48.2
	github.com/aws/aws-sdk-go-v2/config v1.33.23
	github.com/aws/aws-sdk-go-v2/service/sqs v1.53.2
	github.com/chehsunliu/itx/itx-go/itx-contract v0.0.0
	github.com/go-sql-driver/mysql v1.11.0
	github.com/google/uuid v1.6.0
	github.com/jackc/pgx/v5 v5.13.0
	github.com/rabbitmq/amqp091-go v1.17.0
	golang.org/x/sync v0.22.0
)

require (
	filippo.io/edwards25519 v1.2.0 // indirect
	github.com/aws/aws-sdk-go-v2/credentials v1.21.5 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.20.11 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.5.14 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.8.14 // indirect
	github.com/aws/aws-sdk-go-v2/internal/v4a v1.5.14 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.13.27 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.14.14 // indirect
	github.com/aws/aws-sdk-go-v2/service/signin v1.14.2 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.39.7 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.44.7 // indirect
	github.com/aws/aws-sdk-go-v2/service/sts v1.53.2 // indirect
	github.com/aws/smithy-go v1.28.11 // indirect
	github.com/jackc/pgpassfile v1.0.0 // indirect
	github.com/jackc/pgservicefile v0.0.0-20240606120523-5a60cdf6a761 // indirect
	github.com/jackc/puddle/v2 v2.2.3 // indirect
	golang.org/x/text v0.41.0 // indirect
)

replace github.com/chehsunliu/itx/itx-go/itx-contract => ../itx-contract
