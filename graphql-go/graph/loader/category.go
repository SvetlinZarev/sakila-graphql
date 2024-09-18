package loader

import (
	"github.com/jackc/pgx/v5"
	"sakila-graphql-go/graph/model"
)

//var CategoriesScanOrder = []string{
//	model.CategoryTableId,
//	model.CategoryTableColumnName,
//}

func CategoryFromRow(rows pgx.Rows) (model.Category, error) {
	x := model.Category{}
	err := rows.Scan(&x.Id, &x.Name)

	return x, err
}
