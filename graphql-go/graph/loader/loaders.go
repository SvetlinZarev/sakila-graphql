package loader

import (
	"context"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/vikstrous/dataloadgen"
	"sakila-graphql-go/graph/model"
	"sakila-graphql-go/internal/config"
	"time"
)

type ctxKey string

const (
	loadersKey = ctxKey("dataloaders")
)

type Loaders struct {
	LanguageLoader       *dataloadgen.Loader[int, *model.Language]
	CategoryLoader       *dataloadgen.Loader[int, *model.Category]
	ActorLoader          *dataloadgen.Loader[int, *model.Actor]
	FilmLoader           *dataloadgen.Loader[int, *model.Film]
	FilmCategoryIdLoader *dataloadgen.Loader[int, []int]
	FilmActorIdLoader    *dataloadgen.Loader[int, []int]
	ActorFilmIdLoader    *dataloadgen.Loader[int, []int]
}

func NewLoaders(cfg config.DataLoaderConfig, db *pgxpool.Pool) Loaders {
	dl := NewPostgresDataLoader(db)

	return Loaders{
		LanguageLoader: dataloadgen.NewLoader(
			dl.GetLanguages,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),

		CategoryLoader: dataloadgen.NewLoader(
			dl.GetCategories,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),

		FilmLoader: dataloadgen.NewLoader(
			dl.GetFilms,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),

		ActorLoader: dataloadgen.NewLoader(
			dl.GetActors,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),
		FilmCategoryIdLoader: dataloadgen.NewLoader(
			dl.GetCategoryIdsForFilm,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),

		FilmActorIdLoader: dataloadgen.NewLoader(
			dl.GetActorIdsForFilm,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),

		ActorFilmIdLoader: dataloadgen.NewLoader(
			dl.GetFilmIdsForActor,
			dataloadgen.WithBatchCapacity(cfg.MaxBatchSize),
			dataloadgen.WithWait(time.Duration(cfg.DefaultDelayMs)*time.Millisecond),
		),
	}
}

func FromContext(ctx context.Context) *Loaders {
	return ctx.Value(loadersKey).(*Loaders)
}

func SetToContext(ctx context.Context, loaders *Loaders) context.Context {
	return context.WithValue(ctx, loadersKey, loaders)
}

func GetLanguage(ctx context.Context, keys int) (*model.Language, error) {
	l := FromContext(ctx)
	return l.LanguageLoader.Load(ctx, keys)
}

func GetActors(ctx context.Context, keys []int) ([]*model.Actor, error) {
	l := FromContext(ctx)
	return l.ActorLoader.LoadAll(ctx, keys)
}

func GetFilms(ctx context.Context, keys []int) ([]*model.Film, error) {
	l := FromContext(ctx)
	return l.FilmLoader.LoadAll(ctx, keys)
}

func GetCategories(ctx context.Context, keys []int) ([]*model.Category, error) {
	l := FromContext(ctx)
	return l.CategoryLoader.LoadAll(ctx, keys)
}

func GetCategoryIdsForFilm(ctx context.Context, keys int) ([]int, error) {
	l := FromContext(ctx)
	return l.FilmCategoryIdLoader.Load(ctx, keys)
}

func GetActorIdsForFilm(ctx context.Context, keys int) ([]int, error) {
	l := FromContext(ctx)
	return l.FilmActorIdLoader.Load(ctx, keys)
}

func GetFilmIdsForActor(ctx context.Context, keys int) ([]int, error) {
	l := FromContext(ctx)
	return l.ActorFilmIdLoader.Load(ctx, keys)
}
