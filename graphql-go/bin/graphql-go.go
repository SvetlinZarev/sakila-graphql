package main

import (
	"context"
	"github.com/sethvargo/go-envconfig"
	"os"
	"sakila-graphql-go/internal/config"
	"sakila-graphql-go/internal/inits"
	"sakila-graphql-go/internal/log"
	"sakila-graphql-go/internal/server"
)

func main() {
	ctx := context.Background()
	ctx = context.WithValue(ctx, log.RequestIDKey, "N/A")

	cfg := config.ServiceConfig{}
	if err := envconfig.Process(ctx, &cfg); nil != err {
		log.C(ctx).Error("Failed to load configuration", "err", err)
		os.Exit(1)
	}

	db := inits.InitConnectionPool(ctx, cfg.DB)
	defer func() {
		log.C(ctx).Info("Closing connection pool")
		db.Close()
	}()

	if err := db.Ping(ctx); nil != err {
		log.C(ctx).Error("failed to ping db", "error", err)
	}

	server.StartServer(ctx, cfg, db)

	log.C(ctx).Info("Server stopped")
}
