package postgres

import (
	"database/sql"
	"fmt"

	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/post"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/subscription"
	"github.com/chehsunliu/itx/itx-go/itx-contract/repo/user"
	_ "github.com/jackc/pgx/v5/stdlib"
)

type RepoFactoryProps struct {
	Host     string `yaml:"host"`
	Port     int    `yaml:"port"`
	DBName   string `yaml:"db-name"`
	User     string `yaml:"user"`
	Password string `yaml:"password"`
}

type RepoFactory struct {
	db *sql.DB
}

func New(props RepoFactoryProps) (*RepoFactory, error) {
	dsn := fmt.Sprintf("postgres://%s:%s@%s:%d/%s", props.User, props.Password, props.Host, props.Port, props.DBName)
	db, err := sql.Open("pgx", dsn)
	if err != nil {
		return nil, err
	}
	db.SetMaxOpenConns(10)

	return &RepoFactory{db: db}, nil
}

func (f *RepoFactory) CreatePostRepo() post.Repo {
	return &postRepo{db: f.db}
}

func (f *RepoFactory) CreateUserRepo() user.Repo {
	return &userRepo{db: f.db}
}

func (f *RepoFactory) CreateSubscriptionRepo() subscription.Repo {
	return &subscriptionRepo{db: f.db}
}
