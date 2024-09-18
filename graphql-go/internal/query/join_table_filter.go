package query

type JoinTableFilter struct {
	ChildTableFilter TableFilter
	JoinTable        string

	ParentTableJoinColumn string
	ChildTableJoinColumn  string

	JoinTableParentColumn string
	JoinTableChildColumn  string
}

func (f *JoinTableFilter) Accept(v Visitor) {
	v.onJoinTableFilter(f)
}

func (f *JoinTableFilter) __isFilter() {
}
