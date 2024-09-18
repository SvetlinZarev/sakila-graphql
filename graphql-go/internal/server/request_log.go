package server

import (
	"github.com/go-chi/chi/v5/middleware"
	"net/http"
	"sakila-graphql-go/internal/log"
	"time"
)

func Logger(next http.Handler) http.Handler {
	fn := func(w http.ResponseWriter, r *http.Request) {
		log.C(r.Context()).Info("started processing request", "method", r.Method, "uri", r.RequestURI)

		startTime := time.Now()
		ww := middleware.NewWrapResponseWriter(w, r.ProtoMajor)
		defer func() {
			elapsed := time.Since(startTime)
			log.C(r.Context()).Info("finished processing request", "status", ww.Status(), "latency", elapsed.String())
		}()

		next.ServeHTTP(ww, r)

	}

	return http.HandlerFunc(fn)
}
