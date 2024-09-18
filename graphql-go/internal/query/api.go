package query

type InputFilter interface {
	ForTable() string
	AndFilters() []InputFilter
	OrFilters() []InputFilter
	Collect(f *FilterGroup)
}

type Filter interface {
	__isFilter()
	Accept(v Visitor)
}
