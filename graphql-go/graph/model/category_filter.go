package model

import (
	"sakila-graphql-go/graph/filter"
	"sakila-graphql-go/internal/query"
)

type CategoryFilter struct {
	NameEq    *string  `json:"nameEq,omitempty"`
	NameIn    []string `json:"nameIn,omitempty"`
	NameNotEq *string  `json:"nameNotEq,omitempty"`
	NameNotIn []string `json:"nameNotIn,omitempty"`
}

func (c *CategoryFilter) ForTable() string {
	return CategoryTable
}

func (c *CategoryFilter) AndFilters() []query.InputFilter {
	return nil
}

func (c *CategoryFilter) OrFilters() []query.InputFilter {
	return nil
}

func (c *CategoryFilter) Collect(f *query.FilterGroup) {
	filter.AndFilters(f, c)
	filter.OrFilters(f, c)

	filter.Eq(f, CategoryTableColumnName, c.NameEq)
	filter.Neq(f, CategoryTableColumnName, c.NameNotEq)
	filter.IsIn(f, CategoryTableColumnName, c.NameIn)
	filter.IsNotIn(f, CategoryTableColumnName, c.NameNotIn)
}
