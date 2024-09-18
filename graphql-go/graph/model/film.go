package model

const FilmTable = "film"
const FilmTableId = "film_id"
const FilmTableColumnTitle = "title"
const FilmTableColumnDescription = "description"
const FilmTableColumnLength = "length"
const FilmTableColumnLangId = "language_id"
const FilmTableColumnOrigLangId = "original_language_id"

type Film struct {
	Title              string `json:"title"`
	Description        string `json:"description"`
	Length             int    `json:"length"`
	FilmId             int
	LanguageId         int
	OriginalLanguageId *int
}
