package model

const CategoryTable = "category"
const CategoryTableId = "category_id"
const CategoryTableColumnName = "name"

type Category struct {
	Id   int
	Name string `json:"name"`
}
