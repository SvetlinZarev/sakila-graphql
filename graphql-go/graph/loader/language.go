package loader

import (
	"github.com/jackc/pgx/v5"
	"sakila-graphql-go/graph/model"
)

//var LanguageScanOrder = []string{
//	model.LanguageTableId,
//	model.LanguageTableColumnName,
//}

func LanguageFromRow(rows pgx.Rows) (model.Language, error) {
	x := model.Language{}
	err := rows.Scan(&x.Id, &x.Name)

	return x, err
}
