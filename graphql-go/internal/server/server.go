package server

import (
	"context"
	"errors"
	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/jackc/pgx/v5/pgxpool"
	"net/http"
	"os"
	"os/signal"
	"sakila-graphql-go/graph"
	"sakila-graphql-go/internal/config"
	"sakila-graphql-go/internal/log"
	"strconv"
	"sync"
	"syscall"
	"time"
)

const EndpointPlayground = "/playground"
const EndpointGraphql = "/graphql"

func StartServer(ctx context.Context, cfg config.ServiceConfig, db *pgxpool.Pool) {
	schema := graph.NewExecutableSchema(graph.Config{
		Resolvers: &graph.Resolver{
			Db: db,
		},
	})

	gqlServer := handler.NewDefaultServer(schema)
	gqlHandler := InjectDataLoaders(cfg.DataLoader, db, gqlServer)

	r := chi.NewRouter()
	r.Use(RequestID)
	r.Use(Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Timeout(time.Duration(cfg.Server.RequestTimeout) * time.Millisecond))
	r.Get(EndpointPlayground, playground.Handler("GraphQL playground", EndpointGraphql))
	r.Method(http.MethodPost, EndpointGraphql, gqlHandler)

	server := http.Server{
		Addr:    ":" + strconv.Itoa(cfg.Server.Port),
		Handler: r,
	}

	wg := &sync.WaitGroup{}
	wg.Add(1)

	go func() {
		defer wg.Done()

		log.C(ctx).Info("Starting server", "port", cfg.Server.Port)
		if err := server.ListenAndServe(); nil != err && !errors.Is(err, http.ErrServerClosed) {
			log.C(ctx).Error("server failed", "err", err)
		}
	}()

	done := make(chan os.Signal, 1)
	signal.Notify(done, os.Interrupt, syscall.SIGINT, syscall.SIGTERM)

	<-done
	log.C(ctx).Info("received server termination request")
	if err := server.Shutdown(ctx); nil != err {
		log.C(ctx).Error("server shutdown failed", "err", err)
	}

	wg.Wait()
}
