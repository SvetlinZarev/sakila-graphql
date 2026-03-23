use graphql_rust_dyn::config;
use graphql_rust_dyn::config::ServiceConfig;
use graphql_rust_dyn::graphql::schema::assembly::build_graphql_schema;
use graphql_rust_dyn::graphql::schema::model::{
    Cardinality, DirectRelation, Kind, MediatedRelation, Presence, Property, PropertyName,
    Relation, RelationType, ReverseRelation, TableName, Type, TypeName,
};
use graphql_rust_dyn::init::{init_db_pool, init_tracing};
use graphql_rust_dyn::server::{start_server, AppState};
use std::error::Error;

const PREFIX: &str = "CFG";
const CFG_SEPARATOR: &str = "__";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_tracing();

    let cfg = config::load::<_, _, ServiceConfig>(PREFIX, CFG_SEPARATOR)?;

    let db = init_db_pool(&cfg.db)?;
    let state = AppState::new(db.clone());
    //let schema = build_schema(state.clone());

    let types = vec![
        Type {
            name: TypeName::new("employee"),
            desc: Some("Employee of a company".to_owned()),
            table_name: TableName::new("t_employee"),
            key: vec![PropertyName::new("id")],
            properties: vec![
                Property {
                    name: PropertyName::new("id"),
                    kind: Kind::Uuid,
                    presence: Presence::Required,
                    cardinality: Cardinality::One,
                    unique: true,
                    desc: Some("The ID of the instance".to_owned()),
                },
                Property {
                    name: PropertyName::new("group_id"),
                    kind: Kind::Uuid,
                    presence: Presence::Required,
                    cardinality: Cardinality::One,
                    unique: false,
                    desc: Some("The ID of the instance".to_owned()),
                },
            ],
            relations: vec![],
            reverse_relations: vec![],
        },
        Type {
            name: TypeName("demo".to_string()),
            desc: Some("This is a demo type for testing".to_owned()),
            table_name: TableName::new("t_demo"),
            key: vec![PropertyName::new("id")],
            properties: vec![
                Property {
                    name: PropertyName::new("id"),
                    kind: Kind::Uuid,
                    presence: Presence::Required,
                    cardinality: Cardinality::One,
                    unique: true,
                    desc: Some("The ID of the instance".to_owned()),
                },
                Property {
                    name: PropertyName::new("owner_id"),
                    kind: Kind::Uuid,
                    presence: Presence::Required,
                    cardinality: Cardinality::One,
                    unique: true,
                    desc: Some("The ID of the owner of this demo".to_owned()),
                },
                Property {
                    name: PropertyName::new("owner_group"),
                    kind: Kind::Uuid,
                    presence: Presence::Required,
                    cardinality: Cardinality::One,
                    unique: false,
                    desc: Some("The ID of the owner of this demo".to_owned()),
                },
            ],

            relations: vec![
                Relation {
                    name: PropertyName("owner".to_owned()),
                    target: TypeName("employee".to_owned()),
                    kind: RelationType::Direct(DirectRelation {
                        source_property: PropertyName("owner_id".to_owned()),
                        target_property: PropertyName("id".to_owned()),
                        presence: Presence::Required,
                    }),
                    desc: Some("The owner ID".to_owned()),
                },
                Relation {
                    name: PropertyName("owners".to_owned()),
                    target: TypeName("employee".to_owned()),
                    kind: RelationType::Direct(DirectRelation {
                        source_property: PropertyName("owner_group".to_owned()),
                        target_property: PropertyName("group_id".to_owned()),
                        presence: Presence::Required,
                    }),
                    desc: Some("The owner group ID".to_owned()),
                },
                Relation {
                    name: PropertyName("participants".to_owned()),
                    target: TypeName("employee".to_owned()),
                    kind: RelationType::Mediated(MediatedRelation {
                        through: TableName("employee_demo".to_owned()),
                    }),
                    desc: Some("The participants IDs".to_owned()),
                },
            ],
            reverse_relations: vec![ReverseRelation {
                name: PropertyName("company".to_owned()),
                source: TypeName("company".to_owned()),
                source_property: PropertyName("demos".to_owned()),
                desc: Some("The company that presented the demo".to_owned()),
            }],
        },
    ];
    let schema = build_graphql_schema(types)?;

    start_server(cfg, state, schema).await?;

    tracing::info!("Closing connection pool");
    db.close();

    tracing::info!("Server stopped");
    Ok(())
}
