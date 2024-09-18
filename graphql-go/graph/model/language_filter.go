package model

import (
	"sakila-graphql-go/graph/filter"
	"sakila-graphql-go/internal/query"
)

type LanguageFilter struct {
	NameEq       *string  `json:"nameEq,omitempty"`
	NameIn       []string `json:"nameIn,omitempty"`
	NameNotEq    *string  `json:"nameNotEq,omitempty"`
	NameNotIn    []string `json:"nameNotIn,omitempty"`
	NameContains *string  `json:"nameContains,omitempty"`
}

func (l *LanguageFilter) ForTable() string {
	return LanguageTable
}

func (l *LanguageFilter) AndFilters() []query.InputFilter {
	return nil
}

func (l *LanguageFilter) OrFilters() []query.InputFilter {
	return nil
}

func (l *LanguageFilter) Collect(f *query.FilterGroup) {
	filter.AndFilters(f, l)
	filter.OrFilters(f, l)

	filter.Eq(f, CategoryTableColumnName, l.NameEq)
	filter.Neq(f, CategoryTableColumnName, l.NameNotEq)
	filter.IsIn(f, CategoryTableColumnName, l.NameIn)
	filter.IsNotIn(f, CategoryTableColumnName, l.NameNotIn)
}
