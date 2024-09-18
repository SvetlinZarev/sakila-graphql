package query

type Combinator string
type Operation int

const (
	AND Combinator = "AND"
	OR  Combinator = "OR"
)

const (
	Eq Operation = iota
	Neq
	Lt
	Gt
	Lte
	Gte
	In
	NotIn
	IsNull
	IsNotNull
	Contains
)
