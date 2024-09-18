package inits

import (
	"context"
	"fmt"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"sakila-graphql-go/internal/config"
)

func InitConnectionPool(ctx context.Context, cfg config.PoolConfig) *pgxpool.Pool {
	cs := connectionString(&cfg)

	poolCfg, err := pgxpool.ParseConfig(cs)
	if err != nil {
		panic(fmt.Sprintf("failed to parse DB connection string: %s", err))
	}

	configurePassword(&cfg, poolCfg)

	pool, err := pgxpool.NewWithConfig(ctx, poolCfg)
	if err != nil {
		panic(fmt.Sprintf("failed to create connection pool: %s", err))
	}

	return pool
}

func connectionString(cfg *config.PoolConfig) string {
	return fmt.Sprintf(
		"host=%s port=%d user=%s dbname=%s pool_max_conns=%d sslmode=disable ",
		cfg.Host,
		cfg.Port,
		cfg.User,
		cfg.DbName,
		cfg.MaxConn,
	)
}

func configurePassword(cfg *config.PoolConfig, poolCfg *pgxpool.Config) {
	poolCfg.BeforeConnect = func(ctx context.Context, connConfig *pgx.ConnConfig) error {
		connConfig.Password = cfg.Pass
		return nil
	}
}
