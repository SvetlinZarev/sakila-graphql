package loader

import (
	"github.com/jackc/pgx/v5"
	"sakila-graphql-go/graph/model"
)

var ActorScanOrder = []string{
	model.ActorTableId,
	model.ActorFirstName,
	model.ActorLastName,
}

func ActorFromRow(rows pgx.Rows) (model.Actor, error) {
	a := model.Actor{}
	err := rows.Scan(&a.ActorId, &a.FirstName, &a.LastName)

	return a, err
}

func LoadActorsFromRows(rows pgx.Rows) ([]*model.Actor, error) {
	var actors []*model.Actor
	for rows.Next() {
		actor, err := ActorFromRow(rows)
		if err != nil {
			return nil, err
		}

		actors = append(actors, &actor)
	}

	return actors, nil
}
