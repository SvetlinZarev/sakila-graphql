package server

import (
	"github.com/jackc/pgx/v5/pgxpool"
	"net/http"
	"sakila-graphql-go/graph/loader"
	"sakila-graphql-go/internal/config"
)

func InjectDataLoaders(cfg config.DataLoaderConfig, db *pgxpool.Pool, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		l := loader.NewLoaders(cfg, db)
		r = r.WithContext(loader.SetToContext(r.Context(), &l))
		next.ServeHTTP(w, r)
	})
}
