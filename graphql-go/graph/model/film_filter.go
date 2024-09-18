package model

import (
	"sakila-graphql-go/graph/filter"
	"sakila-graphql-go/internal/query"
)

type FilmFilter struct {
	And              []FilmFilter    `json:"and,omitempty"`
	Or               []FilmFilter    `json:"or,omitempty"`
	Actor            *ActorFilter    `json:"actor,omitempty"`
	Category         *CategoryFilter `json:"category,omitempty"`
	Language         *LanguageFilter `json:"language,omitempty"`
	OriginalLanguage *LanguageFilter `json:"originalLanguage,omitempty"`
	TitleEq          *string         `json:"titleEq,omitempty"`
	TitleNotEq       *string         `json:"titleNotEq,omitempty"`
	TitleIn          []string        `json:"titleIn,omitempty"`
	TitleNotIn       []string        `json:"titleNotIn,omitempty"`
	TitleContains    *string         `json:"titleContains,omitempty"`
	LengthEq         *int            `json:"lengthEq,omitempty"`
	LengthGt         *int            `json:"lengthGt,omitempty"`
	LengthGte        *int            `json:"lengthGte,omitempty"`
	LengthLt         *int            `json:"lengthLt,omitempty"`
	LengthLte        *int            `json:"lengthLte,omitempty"`
}

func (f *FilmFilter) ForTable() string {
	return FilmTable
}

func (f *FilmFilter) AndFilters() []query.InputFilter {
	if len(f.And) == 0 {
		return nil
	}

	// https://go.dev/wiki/InterfaceSlice
	x := make([]query.InputFilter, 0, len(f.And))
	for idx := range f.And {
		x[idx] = &f.And[idx]
	}

	return x
}

func (f *FilmFilter) OrFilters() []query.InputFilter {
	if len(f.Or) == 0 {
		return nil
	}

	// https://go.dev/wiki/InterfaceSlice
	x := make([]query.InputFilter, 0, len(f.Or))
	for idx := range f.Or {
		x[idx] = &f.Or[idx]
	}

	return x
}

func (f *FilmFilter) Collect(g *query.FilterGroup) {
	filter.AndFilters(g, f)
	filter.OrFilters(g, f)

	filter.Eq(g, FilmTableColumnTitle, f.TitleEq)
	filter.Neq(g, FilmTableColumnTitle, f.TitleNotEq)
	filter.IsIn(g, FilmTableColumnTitle, f.TitleIn)
	filter.IsNotIn(g, FilmTableColumnTitle, f.TitleNotIn)
	filter.Contains(g, FilmTableColumnTitle, f.TitleContains)

	filter.Eq(g, FilmTableColumnLength, f.LengthEq)
	filter.Lt(g, FilmTableColumnLength, f.LengthLt)
	filter.Lte(g, FilmTableColumnLength, f.LengthLte)
	filter.Gt(g, FilmTableColumnLength, f.LengthGt)
	filter.Gte(g, FilmTableColumnLength, f.LengthGte)

	filter.JoinColumn(g, f.Language, FilmTableColumnLangId, LanguageTableId)
	filter.JoinColumn(g, f.OriginalLanguage, FilmTableColumnOrigLangId, LanguageTableId)

	filter.JoinTable(g, f.Actor, JoinTableFilmActor, JoinTableFilmActorFilmId, JoinTableFilmActorActorId, FilmTableId, ActorTableId)
	filter.JoinTable(g, f.Category, JoinTableFilmCategory, JoinTableFilmCategoryFilmId, JoinTableFilmCategoryCategoryId, FilmTableId, CategoryTableId)
}
