package loader

import (
	"context"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"sakila-graphql-go/graph/model"
)

type PostgresDataLoader struct {
	db *pgxpool.Pool
}

func NewPostgresDataLoader(db *pgxpool.Pool) PostgresDataLoader {
	return PostgresDataLoader{db: db}
}

func (u *PostgresDataLoader) GetActors(ctx context.Context, keys []int) ([]*model.Actor, []error) {
	const query = "SELECT actor_id, first_name, last_name FROM actor WHERE actor_id = ANY($1)"

	return collectTyped(ctx, u.db, query, keys, ActorFromRow, func(m *model.Actor) int {
		return m.ActorId
	})
}

func (u *PostgresDataLoader) GetFilms(ctx context.Context, keys []int) ([]*model.Film, []error) {
	const query = "SELECT film_id, title, description, length, language_id, original_language_id FROM film WHERE film_id = ANY($1)"

	return collectTyped(ctx, u.db, query, keys, FilmFromRow, func(m *model.Film) int {
		return m.FilmId
	})
}

func (u *PostgresDataLoader) GetLanguages(ctx context.Context, keys []int) ([]*model.Language, []error) {
	const query = "SELECT language_id, name FROM language WHERE language_id = ANY($1)"

	return collectTyped(ctx, u.db, query, keys, LanguageFromRow, func(m *model.Language) int {
		return m.Id
	})
}

func (u *PostgresDataLoader) GetCategories(ctx context.Context, keys []int) ([]*model.Category, []error) {
	const query = "SELECT c.category_id, c.name FROM category c  WHERE c.category_id = ANY($1)"

	return collectTyped(ctx, u.db, query, keys, CategoryFromRow, func(m *model.Category) int {
		return m.Id
	})
}

func (u *PostgresDataLoader) GetCategoryIdsForFilm(ctx context.Context, filmIds []int) ([][]int, []error) {
	const query = "SELECT film_id, category_id FROM film_category  WHERE film_id = ANY($1)"

	return collectForId(ctx, u.db, query, filmIds)

}

func (u *PostgresDataLoader) GetActorIdsForFilm(ctx context.Context, filmIds []int) ([][]int, []error) {
	const query = "SELECT film_id, actor_id FROM film_actor  WHERE film_id = ANY($1)"

	return collectForId(ctx, u.db, query, filmIds)
}

func (u *PostgresDataLoader) GetFilmIdsForActor(ctx context.Context, actorIds []int) ([][]int, []error) {
	const query = "SELECT actor_id, film_id FROM film_actor  WHERE actor_id = ANY($1)"

	return collectForId(ctx, u.db, query, actorIds)
}

func collectTyped[T any, K comparable](ctx context.Context, db *pgxpool.Pool, queryStr string, keys []K, loadFn func(rows pgx.Rows) (T, error), keyFn func(*T) K) ([]*T, []error) {
	rows, err := db.Query(ctx, queryStr, keys)
	if err != nil {
		return nil, []error{err}
	}

	defer rows.Close()

	idMap := make(map[K]*T)
	for rows.Next() {
		value, err := loadFn(rows)
		if err != nil {
			errs := make([]error, 0, len(keys))
			for range keys {
				errs = append(errs, err)
			}

			return nil, errs
		}

		idMap[keyFn(&value)] = &value
	}

	values := make([]*T, 0, len(keys))
	for _, key := range keys {
		values = append(values, idMap[key])
	}

	return values, nil
}

func collectForId(ctx context.Context, db *pgxpool.Pool, queryStr string, keys []int) ([][]int, []error) {
	rows, err := db.Query(ctx, queryStr, keys)
	if err != nil {
		return nil, []error{err}
	}

	defer rows.Close()

	idMap := make(map[int][]int)
	for rows.Next() {
		var key, value int

		if err := rows.Scan(&key, &value); nil != err {
			errs := make([]error, 0, len(keys))
			for range keys {
				errs = append(errs, err)
			}

			return nil, errs
		}

		idMap[key] = append(idMap[key], value)
	}

	ids := make([][]int, 0, len(keys))
	for _, key := range keys {
		ids = append(ids, idMap[key])
	}

	return ids, nil
}
