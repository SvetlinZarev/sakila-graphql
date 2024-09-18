package query

type Visitor interface {
	onTableFilter(f *TableFilter)
	onFilterGroup(f *FilterGroup)
	onValueFilter(f *ValueFilter)
	onJoinColumnFilter(f *JoinColumnFilter)
	onJoinTableFilter(f *JoinTableFilter)
}
