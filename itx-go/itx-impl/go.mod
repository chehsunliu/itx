module github.com/chehsunliu/itx/itx-go/itx-impl

go 1.25.4

require (
	github.com/aws/aws-sdk-go-v2 v1.43.8
	github.com/aws/aws-sdk-go-v2/config v1.32.39
	github.com/aws/aws-sdk-go-v2/service/sqs v1.46.8
	github.com/chehsunliu/itx/itx-go/itx-contract v0.0.0
	github.com/go-sql-driver/mysql v1.10.0
	github.com/google/uuid v1.6.0
	github.com/jackc/pgx/v5 v5.10.0
	github.com/rabbitmq/amqp091-go v1.14.0
	golang.org/x/sync v0.22.0
)

require (
	filippo.io/edwards25519 v1.2.0 // indirect
	github.com/aws/aws-sdk-go-v2/credentials v1.19.38 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.18.39 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.4.39 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.7.39 // indirect
	github.com/aws/aws-sdk-go-v2/internal/v4a v1.4.40 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.13.18 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.13.39 // indirect
	github.com/aws/aws-sdk-go-v2/service/signin v1.5.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.33.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.38.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/sts v1.45.8 // indirect
	github.com/aws/smithy-go v1.27.10 // indirect
	github.com/jackc/pgpassfile v1.0.0 // indirect
	github.com/jackc/pgservicefile v0.0.0-20240606120523-5a60cdf6a761 // indirect
	github.com/jackc/puddle/v2 v2.2.2 // indirect
	golang.org/x/text v0.34.0 // indirect
)

replace github.com/chehsunliu/itx/itx-go/itx-contract => ../itx-contract
