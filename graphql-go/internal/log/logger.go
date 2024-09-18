package log

import (
	"context"
	"log/slog"
	"os"
)

type ctxKeyRequestID int

const RequestIDKey ctxKeyRequestID = 0

var logger *slog.Logger

func init() {
	logger = slog.New(slog.NewTextHandler(os.Stdout, nil))
}

func C(ctx context.Context) *slog.Logger {
	requestId := ctx.Value(RequestIDKey)
	return logger.With("request_id", requestId)
}
