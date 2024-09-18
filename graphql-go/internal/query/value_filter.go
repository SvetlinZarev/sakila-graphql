package query

type ValueFilter struct {
	Operation Operation
	Column    string
	Value     any
}

func (f *ValueFilter) Accept(v Visitor) {
	v.onValueFilter(f)
}

func (f *ValueFilter) __isFilter() {
}
