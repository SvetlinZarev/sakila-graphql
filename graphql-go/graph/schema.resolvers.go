package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.
// Code generated by github.com/99designs/gqlgen version v0.17.54

import (
	"context"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"sakila-graphql-go/graph/loader"
	"sakila-graphql-go/graph/model"
	"sakila-graphql-go/internal/log"
	"sakila-graphql-go/internal/query"
	"sakila-graphql-go/internal/util"
)

// Films is the resolver for the films field.
func (r *actorResolver) Films(ctx context.Context, obj *model.Actor, filter *model.FilmFilter) ([]*model.Film, error) {
	if filter == nil {
		ids, err := loader.GetFilmIdsForActor(ctx, obj.ActorId)
		if err != nil {
			return nil, err
		}

		return loader.GetFilms(ctx, ids)
	}

	jt := query.JoinedTable{
		JoinTable:             model.JoinTableFilmActor,
		JoinTableJoinCol:      model.JoinTableFilmActorFilmId,
		DataTableJoinCol:      model.FilmTableId,
		JoinTableFilterCol:    model.ActorTableId,
		JoinTableFilterColVal: obj.ActorId,
	}

	rows, err := process(ctx, r.Db, model.FilmTable, loader.FilmScanOrder, filter, &jt)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return loader.LoadFilmsFromRows(rows)
}

// Actors is the resolver for the actors field.
func (r *filmResolver) Actors(ctx context.Context, obj *model.Film, filter *model.ActorFilter) ([]*model.Actor, error) {
	if filter == nil {
		ids, err := loader.GetActorIdsForFilm(ctx, obj.FilmId)
		if err != nil {
			return nil, err
		}

		return loader.GetActors(ctx, ids)
	}

	jt := &query.JoinedTable{
		JoinTable:             model.JoinTableFilmActor,
		JoinTableJoinCol:      model.JoinTableFilmActorActorId,
		DataTableJoinCol:      model.ActorTableId,
		JoinTableFilterCol:    model.FilmTableId,
		JoinTableFilterColVal: obj.FilmId,
	}

	rows, err := process(ctx, r.Db, model.ActorTable, loader.ActorScanOrder, filter, jt)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return loader.LoadActorsFromRows(rows)
}

// Categories is the resolver for the categories field.
func (r *filmResolver) Categories(ctx context.Context, obj *model.Film) ([]*model.Category, error) {
	ids, err := loader.GetCategoryIdsForFilm(ctx, obj.FilmId)
	if err != nil {
		return nil, err
	}

	return loader.GetCategories(ctx, ids)
}

// Language is the resolver for the language field.
func (r *filmResolver) Language(ctx context.Context, obj *model.Film) (*model.Language, error) {
	return loader.GetLanguage(ctx, obj.LanguageId)
}

// OriginalLanguage is the resolver for the originalLanguage field.
func (r *filmResolver) OriginalLanguage(ctx context.Context, obj *model.Film) (*model.Language, error) {
	if obj.OriginalLanguageId == nil {
		return nil, nil
	}

	return loader.GetLanguage(ctx, *obj.OriginalLanguageId)
}

// Actors is the resolver for the actors field.
func (r *queryResolver) Actors(ctx context.Context, filter *model.ActorFilter) ([]*model.Actor, error) {
	rows, err := process(ctx, r.Db, model.ActorTable, loader.ActorScanOrder, filter, nil)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return loader.LoadActorsFromRows(rows)
}

// Films is the resolver for the films field.
func (r *queryResolver) Films(ctx context.Context, filter *model.FilmFilter) ([]*model.Film, error) {
	rows, err := process(ctx, r.Db, model.FilmTable, loader.FilmScanOrder, filter, nil)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return loader.LoadFilmsFromRows(rows)
}

// Actor returns ActorResolver implementation.
func (r *Resolver) Actor() ActorResolver { return &actorResolver{r} }

// Film returns FilmResolver implementation.
func (r *Resolver) Film() FilmResolver { return &filmResolver{r} }

// Query returns QueryResolver implementation.
func (r *Resolver) Query() QueryResolver { return &queryResolver{r} }

type actorResolver struct{ *Resolver }
type filmResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }

func process(ctx context.Context, db *pgxpool.Pool, tableName string, scanOrder []string, filter query.InputFilter, joinTable *query.JoinedTable) (pgx.Rows, error) {
	v := query.NewSqlVisitor()
	if joinTable != nil {
		v = query.NewSqlVisitorWithJoinedTable(joinTable)
	}

	tf := query.TableFilter{
		Table: tableName,
		FilterGroup: query.FilterGroup{
			Combinator: query.AND,
		},
	}

	if !util.IsNilPointer(filter) {
		filter.Collect(&tf.FilterGroup)
	}

	sql, params := v.Translate(tf, scanOrder)
	log.C(ctx).Debug("processing query filters", "sql", sql, "params", params)

	rows, err := db.Query(ctx, sql, params...)
	if err != nil {
		log.C(ctx).Error("query failed", "sql", sql, "error", err)
		return nil, err
	}

	return rows, err
}
