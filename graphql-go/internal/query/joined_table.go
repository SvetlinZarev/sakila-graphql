package query

type JoinedTable struct {
	JoinTable             string
	JoinTableJoinCol      string
	DataTableJoinCol      string
	JoinTableFilterCol    string
	JoinTableFilterColVal any
}
