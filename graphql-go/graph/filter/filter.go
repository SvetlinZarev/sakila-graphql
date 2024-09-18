package filter

import (
	"sakila-graphql-go/internal/query"
	"sakila-graphql-go/internal/util"
)

func AndFilters(g *query.FilterGroup, f query.InputFilter) {
	and := f.AndFilters()

	var fg *query.FilterGroup
	withNewAlloc := false

	if g.Combinator == query.AND {
		fg = g
	} else {
		withNewAlloc = true
		fg = &query.FilterGroup{
			Combinator: query.AND,
		}
	}

	if len(and) > 0 {
		for _, filter := range and {
			filter.Collect(fg)
		}
	}

	if withNewAlloc {
		g.AddGroup(*fg)
	}
}

func OrFilters(g *query.FilterGroup, f query.InputFilter) {
	and := f.AndFilters()

	var fg *query.FilterGroup
	withNewAlloc := false

	if g.Combinator == query.OR {
		fg = g
	} else {
		withNewAlloc = true
		fg = &query.FilterGroup{
			Combinator: query.OR,
		}
	}

	if len(and) > 0 {
		for _, filter := range and {
			filter.Collect(fg)
		}
	}

	if withNewAlloc {
		g.AddGroup(*fg)
	}
}

func Eq(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Eq, value)
}

func Neq(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Neq, value)
}

func Lt(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Lt, value)
}

func Lte(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Lte, value)
}

func Gt(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Gt, value)
}

func Gte(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Gte, value)
}

func IsIn(g *query.FilterGroup, column string, value any) {
	op(g, column, query.In, value)
}

func IsNotIn(g *query.FilterGroup, column string, value any) {
	op(g, column, query.NotIn, value)
}

func IsNull(g *query.FilterGroup, column string) {
	op(g, column, query.IsNull, "")
}

func IsNotNull(g *query.FilterGroup, column string) {
	op(g, column, query.IsNotNull, "")
}

func Contains(g *query.FilterGroup, column string, value any) {
	op(g, column, query.Contains, value)
}

func op(g *query.FilterGroup, column string, operation query.Operation, value any) {
	if !util.IsNilPointer(value) {
		f := query.ValueFilter{
			Operation: operation,
			Column:    column,
			Value:     value,
		}

		g.AddFilter(&f)
	}
}

func JoinTable(
	g *query.FilterGroup,
	childFilter query.InputFilter,
	joinTable string,
	jtParentCol string,
	jtChildCol string,
	parentJoinCol string,
	childJoinCol string,
) {
	if !util.IsNilPointer(childFilter) {
		tf := query.TableFilter{
			Table: childFilter.ForTable(),
			FilterGroup: query.FilterGroup{
				Combinator: query.AND,
			},
		}
		childFilter.Collect(&tf.FilterGroup)

		jtf := query.JoinTableFilter{
			ChildTableFilter:      tf,
			JoinTable:             joinTable,
			ParentTableJoinColumn: parentJoinCol,
			ChildTableJoinColumn:  childJoinCol,
			JoinTableParentColumn: jtParentCol,
			JoinTableChildColumn:  jtChildCol,
		}

		g.AddFilter(&jtf)
	}
}

func JoinColumn(
	g *query.FilterGroup,
	childFilter query.InputFilter,
	parentCol string,
	childCol string,
) {
	if !util.IsNilPointer(childFilter) {
		tf := query.TableFilter{
			Table: childFilter.ForTable(),
			FilterGroup: query.FilterGroup{
				Combinator: query.AND,
			},
		}
		childFilter.Collect(&tf.FilterGroup)

		jcf := query.JoinColumnFilter{
			Filter:       tf,
			ParentColumn: parentCol,
			ChildColumn:  childCol,
		}

		g.AddFilter(&jcf)
	}
}
