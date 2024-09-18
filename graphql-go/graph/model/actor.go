package model

const ActorTable = "actor"
const ActorTableId = "actor_id"
const ActorFirstName = "first_name"
const ActorLastName = "last_name"

type Actor struct {
	FirstName string `json:"firstName"`
	LastName  string `json:"lastName"`
	ActorId   int
}
