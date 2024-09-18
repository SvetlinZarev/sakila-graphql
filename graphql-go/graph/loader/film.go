package loader

import (
	"github.com/jackc/pgx/v5"
	"sakila-graphql-go/graph/model"
)

var FilmScanOrder = []string{
	model.FilmTableId,
	model.FilmTableColumnTitle,
	model.FilmTableColumnDescription,
	model.FilmTableColumnLength,
	model.FilmTableColumnLangId,
	model.FilmTableColumnOrigLangId,
}

func FilmFromRow(rows pgx.Rows) (model.Film, error) {
	f := model.Film{}
	err := rows.Scan(&f.FilmId, &f.Title, &f.Description, &f.Length, &f.LanguageId, &f.OriginalLanguageId)

	return f, err
}

func LoadFilmsFromRows(rows pgx.Rows) ([]*model.Film, error) {
	var films []*model.Film
	for rows.Next() {
		film, err := FilmFromRow(rows)
		if err != nil {
			return nil, err
		}

		films = append(films, &film)
	}

	return films, nil
}
