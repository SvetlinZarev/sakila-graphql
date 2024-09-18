package model

import (
	"sakila-graphql-go/graph/filter"
	"sakila-graphql-go/internal/query"
)

type ActorFilter struct {
	And            []ActorFilter `json:"and,omitempty"`
	Or             []ActorFilter `json:"or,omitempty"`
	Film           *FilmFilter   `json:"film,omitempty"`
	FirstNameEq    *string       `json:"firstNameEq,omitempty"`
	FirstNameIn    []string      `json:"firstNameIn,omitempty"`
	FirstNameNotEq *string       `json:"firstNameNotEq,omitempty"`
	FirstNameNotIn []string      `json:"firstNameNotIn,omitempty"`
	LastNameEq     *string       `json:"lastNameEq,omitempty"`
	LastNameIn     []string      `json:"lastNameIn,omitempty"`
	LastNameNotEq  *string       `json:"lastNameNotEq,omitempty"`
	LastNameNotIn  []string      `json:"lastNameNotIn,omitempty"`
}

func (a *ActorFilter) ForTable() string {
	return ActorTable
}

func (a *ActorFilter) AndFilters() []query.InputFilter {
	if len(a.And) == 0 {
		return nil
	}

	// https://go.dev/wiki/InterfaceSlice
	f := make([]query.InputFilter, 0, len(a.And))
	for idx := range a.And {
		f[idx] = &a.And[idx]
	}

	return f
}

func (a *ActorFilter) OrFilters() []query.InputFilter {
	if len(a.Or) == 0 {
		return nil
	}

	// https://go.dev/wiki/InterfaceSlice
	f := make([]query.InputFilter, 0, len(a.Or))
	for idx := range a.Or {
		f[idx] = &a.Or[idx]
	}

	return f
}

func (a *ActorFilter) Collect(f *query.FilterGroup) {
	filter.AndFilters(f, a)
	filter.OrFilters(f, a)

	filter.Eq(f, ActorFirstName, a.FirstNameEq)
	filter.Neq(f, ActorFirstName, a.FirstNameNotEq)
	filter.IsIn(f, ActorFirstName, a.FirstNameIn)
	filter.IsNotIn(f, ActorFirstName, a.FirstNameNotIn)

	filter.Eq(f, ActorLastName, a.LastNameEq)
	filter.Neq(f, ActorLastName, a.LastNameNotEq)
	filter.IsIn(f, ActorLastName, a.LastNameIn)
	filter.IsNotIn(f, ActorLastName, a.LastNameNotIn)

	filter.JoinTable(f, a.Film, JoinTableFilmActor, JoinTableFilmActorActorId, JoinTableFilmActorFilmId, ActorTableId, FilmTableId)
}
