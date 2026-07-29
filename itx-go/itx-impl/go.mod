module github.com/chehsunliu/itx/itx-go/itx-impl

go 1.25.4

require (
	github.com/aws/aws-sdk-go-v2 v1.43.1
	github.com/aws/aws-sdk-go-v2/config v1.32.32
	github.com/aws/aws-sdk-go-v2/service/sqs v1.46.1
	github.com/chehsunliu/itx/itx-go/itx-contract v0.0.0
	github.com/go-sql-driver/mysql v1.10.0
	github.com/google/uuid v1.6.0
	github.com/jackc/pgx/v5 v5.10.0
	github.com/rabbitmq/amqp091-go v1.13.0
	golang.org/x/sync v0.22.0
)

require (
	filippo.io/edwards25519 v1.2.0 // indirect
	github.com/aws/aws-sdk-go-v2/credentials v1.19.31 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.18.32 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.4.32 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.7.32 // indirect
	github.com/aws/aws-sdk-go-v2/internal/v4a v1.4.33 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.13.14 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.13.32 // indirect
	github.com/aws/aws-sdk-go-v2/service/signin v1.5.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.33.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.38.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/sts v1.45.1 // indirect
	github.com/aws/smithy-go v1.27.5 // indirect
	github.com/jackc/pgpassfile v1.0.0 // indirect
	github.com/jackc/pgservicefile v0.0.0-20240606120523-5a60cdf6a761 // indirect
	github.com/jackc/puddle/v2 v2.2.2 // indirect
	golang.org/x/text v0.34.0 // indirect
)

replace github.com/chehsunliu/itx/itx-go/itx-contract => ../itx-contract
