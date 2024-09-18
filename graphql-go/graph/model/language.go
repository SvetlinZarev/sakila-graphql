package model

const LanguageTable = "language"
const LanguageTableId = "language_id"
const LanguageTableColumnName = "name"

type Language struct {
	Id   int
	Name string `json:"name"`
}
