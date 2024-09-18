package query

import (
	"fmt"
	"strings"
)

const defaultInitialBufferCapacity = 1024

func NewSqlVisitor() SqlVisitor {
	return SqlVisitor{}
}

func NewSqlVisitorWithJoinedTable(jt *JoinedTable) SqlVisitor {
	return SqlVisitor{
		joinedTable: jt,
	}
}

type SqlVisitor struct {
	joinedTable  *JoinedTable
	nextTable    int
	currentTable []int
	query        strings.Builder
	params       []any
}

func (s *SqlVisitor) Translate(filter TableFilter, sel []string) (string, []any) {
	s.query.Grow(defaultInitialBufferCapacity)
	currentTableId := s.pushNewTable()

	s.query.WriteString("SELECT ")
	if len(sel) == 0 {
		s.query.WriteString("*")
	} else {
		for idx, sl := range sel {
			if idx > 0 {
				s.query.WriteString(", ")
			}

			s.query.WriteString(fmt.Sprintf("T%d.%s", currentTableId, sl))
		}
	}

	s.query.WriteString(" FROM ")
	if s.joinedTable != nil {
		s.query.WriteString(fmt.Sprintf(
			"%s AS J INNER JOIN %s AS T%d ON J.%s = T%d.%s",
			s.joinedTable.JoinTable,
			filter.Table,
			currentTableId,
			s.joinedTable.JoinTableJoinCol,
			currentTableId,
			s.joinedTable.DataTableJoinCol,
		))
	} else {
		s.query.WriteString(fmt.Sprintf("%s AS T%d", filter.Table, currentTableId))
	}

	if !filter.FilterGroup.IsEmpty() || s.joinedTable != nil {
		s.query.WriteString(" WHERE ")
	}

	if !filter.FilterGroup.IsEmpty() {
		filter.Accept(s)
	}

	if s.joinedTable != nil {
		if !filter.FilterGroup.IsEmpty() {
			s.query.WriteString(" AND ")
		}

		param := s.addParam(s.joinedTable.JoinTableFilterColVal)
		s.query.WriteString(fmt.Sprintf("J.%s = $%d", s.joinedTable.JoinTableFilterCol, param))
	}

	return s.query.String(), s.params
}

func (s *SqlVisitor) pushNewTable() int {
	s.currentTable = append(s.currentTable, s.nextTable)
	s.nextTable += 1
	return s.nextTable - 1
}

func (s *SqlVisitor) popOldTable() {
	s.currentTable = s.currentTable[:len(s.currentTable)-1]
}

func (s *SqlVisitor) currentTableId() int {
	return s.currentTable[len(s.currentTable)-1]
}

func (s *SqlVisitor) addParam(p any) int {
	s.params = append(s.params, p)
	return len(s.params)
}

func (s *SqlVisitor) onTableFilter(f *TableFilter) {
	f.FilterGroup.Accept(s)
}

func (s *SqlVisitor) onFilterGroup(f *FilterGroup) {
	writeBrackets := (len(f.Filters) + len(f.Groups)) > 1
	if writeBrackets {
		s.query.WriteString("(")
	}

	for idx := range f.Filters {
		if idx > 0 {
			s.query.WriteString(" ")
			s.query.WriteString(string(f.Combinator))
			s.query.WriteString(" ")
		}

		f.Filters[idx].Accept(s)
	}

	for idx := range f.Groups {
		if idx > 0 || len(f.Filters) > 0 {
			s.query.WriteString(" ")
			s.query.WriteString(string(f.Combinator))
			s.query.WriteString(" ")
		}

		f.Groups[idx].Accept(s)
	}

	if writeBrackets {
		s.query.WriteString(")")
	}
}

func (s *SqlVisitor) onValueFilter(f *ValueFilter) {
	s.query.WriteString(fmt.Sprintf("T%d.%s ", s.currentTableId(), f.Column))

	switch f.Operation {
	case Eq:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("= $%d", param))
		}
	case Neq:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("<> $%d", param))
		}
	case Lt:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("< $%d", param))
		}
	case Gt:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("> $%d", param))
		}
	case Lte:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("<= $%d", param))
		}
	case Gte:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf(">= $%d", param))
		}
	case In:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("= ANY($%d)", param))
		}
	case NotIn:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("<> ANY($%d)", param))
		}
	case IsNull:
		{
			s.query.WriteString("IS NULL")
		}
	case IsNotNull:
		{
			s.query.WriteString("IS NOT NULL")
		}
	case Contains:
		{
			param := s.addParam(f.Value)
			s.query.WriteString(fmt.Sprintf("LIKE '%%' || $%d || '%%'", param))
		}
	default:
		panic(fmt.Sprintf("operation %d is not supported", f.Operation))
	}
}

func (s *SqlVisitor) onJoinColumnFilter(f *JoinColumnFilter) {
	parentTableId := s.currentTableId()
	childTableId := s.pushNewTable()

	s.query.WriteString(fmt.Sprintf(
		"EXISTS(SELECT TRUE FROM %s AS T%d WHERE T%d.%s = T%d.%s",
		f.Filter.Table,
		childTableId,
		childTableId,
		f.ChildColumn,
		parentTableId,
		f.ParentColumn,
	))

	if !f.Filter.FilterGroup.IsEmpty() {
		s.query.WriteString(" AND ")
		f.Filter.Accept(s)
	}

	s.query.WriteString(")")
	s.popOldTable()
}

func (s *SqlVisitor) onJoinTableFilter(f *JoinTableFilter) {
	parentTableId := s.currentTableId()
	joinTableId := s.pushNewTable()
	childTableId := s.pushNewTable()

	s.query.WriteString(fmt.Sprintf(
		"EXISTS(SELECT TRUE FROM %s AS T%d INNER JOIN %s AS T%d ON T%d.%s = T%d.%s WHERE T%d.%s = T%d.%s",
		f.JoinTable,
		joinTableId,
		f.ChildTableFilter.Table,
		childTableId,
		childTableId,
		f.ChildTableJoinColumn,
		joinTableId,
		f.JoinTableChildColumn,
		joinTableId,
		f.JoinTableParentColumn,
		parentTableId,
		f.ParentTableJoinColumn,
	))

	if !f.ChildTableFilter.FilterGroup.IsEmpty() {
		s.query.WriteString(" AND ")
		f.ChildTableFilter.Accept(s)
	}

	s.query.WriteString(")")
	s.popOldTable()
	s.popOldTable()
}
