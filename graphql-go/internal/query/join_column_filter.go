package query

type JoinColumnFilter struct {
	Filter       TableFilter
	ParentColumn string
	ChildColumn  string
}

func (f *JoinColumnFilter) Accept(v Visitor) {
	v.onJoinColumnFilter(f)
}

func (f *JoinColumnFilter) __isFilter() {
}
