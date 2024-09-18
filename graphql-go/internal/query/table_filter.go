package query

type TableFilter struct {
	Table       string
	FilterGroup FilterGroup
}

func (f *TableFilter) Accept(v Visitor) {
	v.onTableFilter(f)
}
