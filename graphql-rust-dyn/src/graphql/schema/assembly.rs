use crate::graphql::schema::model::{
    Cardinality, DirectRelation, Kind, Presence, Property, Relation, RelationType, Type, TypeName,
};
use crate::graphql::schema::resolver::default_resolver;
use async_graphql::dynamic::*;
use foldhash::{HashMap, HashMapExt};

const TYPE_NAME_QUERY: &'static str = "query";
const TYPE_NAME_PAGE_INFO: &'static str = "page_info";
const TYPE_NAME_METADATA: &'static str = "metadata";

pub fn build_graphql_schema(metamodel: Vec<Type>) -> Result<Schema, SchemaError> {
    let mut schema_builder = Schema::build(TYPE_NAME_QUERY, None, None);
    let mut query = Object::new(TYPE_NAME_QUERY);

    for ty in build_common_types() {
        schema_builder = schema_builder.register(ty);
    }

    let mut types = HashMap::new();
    for ty in &metamodel {
        types.insert(&ty.name, ty);
    }

    for ty in &metamodel {
        let mut obj = Object::new(ty.name.clone());
        if let Some(desc) = ty.desc.as_ref() {
            obj = obj.description(desc);
        }

        let conn = build_connection_for_type(ty);
        schema_builder = schema_builder.register(conn);

        let edge = build_connection_edge_for_type(ty);
        schema_builder = schema_builder.register(edge);

        let filter = build_type_filter_input(ty);
        schema_builder = schema_builder.register(filter);

        let accessor = build_accessor_by_filter(ty, &format!("{}_by_filter", ty.name));
        query = query.field(accessor);

        for property in &ty.properties {
            obj = obj.field(build_property(property));
            if property.unique {
                query = query.field(build_accessor_by_key(ty, property));
            }
        }

        for relation in &ty.relations {
            let accessor = build_relation(&types, ty, relation)?;
            obj = obj.field(accessor);
        }

        //
        // for relation in &ty.reverse_relations {
        //     obj = obj.field(build_reverse_relation(relation));
        // }

        schema_builder = schema_builder.register(obj);
    }

    schema_builder.register(query).finish()
}

fn build_common_types() -> Vec<Object> {
    vec![build_metadata_type(), build_page_info_type()]
}

fn build_metadata_type() -> Object {
    let mut obj = Object::new(TYPE_NAME_METADATA);
    obj = obj.field(Field::new(
        "created_at",
        TypeRef::named_nn(TypeRef::STRING),
        default_resolver,
    ));

    obj = obj.field(Field::new(
        "updated_at",
        TypeRef::named_nn(TypeRef::STRING),
        default_resolver,
    ));

    obj
}

fn build_page_info_type() -> Object {
    let mut obj = Object::new(TYPE_NAME_PAGE_INFO);
    obj = obj.field(Field::new(
        "has_next_page",
        TypeRef::named_nn(TypeRef::BOOLEAN),
        default_resolver,
    ));

    obj = obj.field(Field::new(
        "page_size",
        TypeRef::named_nn(TypeRef::INT),
        default_resolver,
    ));

    obj = obj.field(Field::new(
        "cursor",
        TypeRef::named_nn(TypeRef::STRING),
        default_resolver,
    ));

    obj
}

fn build_property(property: &Property) -> Field {
    let type_ref = get_type_ref(property.kind, property.presence, property.cardinality);
    let mut field = Field::new(property.name.clone(), type_ref, default_resolver);
    if let Some(desc) = property.desc.as_ref() {
        field = field.description(desc);
    }

    field
}

fn build_accessor_by_key(ty: &Type, property: &Property) -> Field {
    let name = format!("{}_by_{}", &ty.name, &property.name);
    let type_ref = TypeRef::named(ty.name.clone());
    let mut field = Field::new(name, type_ref, default_resolver);

    let filter = InputValue::new(
        property.name.clone(),
        get_type_ref(property.kind, property.presence, property.cardinality),
    );

    field = field.argument(filter);
    field = field.description(format!(
        "Retrieve an instance of type '{}' by its key property '{}' ",
        ty.name, property.name
    ));

    field
}

fn build_accessor_by_filter(ty: &Type, accessor_name: &str) -> Field {
    let mut accessor = Field::new(
        accessor_name,
        TypeRef::named_nn_list(connection_name_for_type(ty)),
        default_resolver,
    );
    accessor = accessor.argument(InputValue::new(
        "filter",
        TypeRef::named(filter_input_name_for_type(&ty.name)),
    ));

    accessor
}

fn build_type_filter_input(ty: &Type) -> InputObject {
    const SUFFIXES_COMPARISON: &[&str] = &["lt", "lte", "gt", "gte", "eq", "neq"];
    const SUFFIX_IS_NULL: &str = "is_null";
    const SUFFIX_CONTAINS: &str = "contains";
    const SUFFIX_NOT_CONTAINS: &str = "not_contains";

    let mut filter = InputObject::new(filter_input_name_for_type(&ty.name));
    let type_name = filter.type_name().to_owned();

    filter = filter.field(InputValue::new("AND", TypeRef::named_nn_list(&type_name)));
    filter = filter.field(InputValue::new("OR", TypeRef::named_nn_list(&type_name)));

    for property in &ty.properties {
        match property.cardinality {
            Cardinality::One => {
                for &suffix in SUFFIXES_COMPARISON {
                    let field = InputValue::new(
                        format!("{}_{}", property.name, suffix),
                        get_type_ref(property.kind, Presence::Optional, Cardinality::One),
                    );

                    filter = filter.field(field);
                }
            }

            Cardinality::Many => {
                let contains = InputValue::new(
                    format!("{}_{}", property.name, SUFFIX_CONTAINS),
                    get_type_ref(property.kind, Presence::Optional, Cardinality::One),
                );

                let not_contains = InputValue::new(
                    format!("{}_{}", property.name, SUFFIX_NOT_CONTAINS),
                    get_type_ref(property.kind, Presence::Optional, Cardinality::One),
                );

                filter = filter.field(contains);
                filter = filter.field(not_contains);
            }
        }

        if Presence::Optional == property.presence {
            let is_null = InputValue::new(
                format!("{}_{}", property.name, SUFFIX_IS_NULL),
                get_type_ref(Kind::Boolean, Presence::Optional, Cardinality::One),
            );

            filter = filter.field(is_null);
        }
    }

    for rel in ty.relations.iter() {
        let target = &rel.target;
        let input = InputValue::new(
            rel.name.clone(),
            TypeRef::named(filter_input_name_for_type(target)),
        );

        filter = filter.field(input);
    }

    filter
}

fn build_relation(
    types: &HashMap<&TypeName, &Type>,
    source: &Type,
    relation: &Relation,
) -> Result<Field, SchemaError> {
    match &relation.kind {
        RelationType::Direct(spec) => build_direct_relation(types, source, relation, spec),
        RelationType::Mediated(_) => build_mediated_relation(types, relation),
    }
}

fn build_direct_relation(
    types: &HashMap<&TypeName, &Type>,
    source_type: &Type,
    relation: &Relation,
    spec: &DirectRelation,
) -> Result<Field, SchemaError> {
    let target_type = get_target_type(relation, types)?;

    let target_property = target_type
        .properties
        .iter()
        .find(|p| p.name == spec.target_property)
        .ok_or_else(|| SchemaError(format!("Relation {} refers to type {} via target property {}, which is not present in the target type", relation.name, relation.target, spec.target_property)))?;

    let source_property = source_type
        .properties
        .iter()
        .find(|p| p.name == spec.source_property)
        .ok_or_else(|| SchemaError(format!("Relation {} refers to type {} via source property {}, which is not present in the source type", relation.name, relation.target, spec.source_property)))?;

    let accessor = match direct_relation_cardinality(target_property) {
        Cardinality::One => {
            build_direct_relation_to_one(relation, spec, source_property, target_type)
        }

        Cardinality::Many => build_accessor_by_filter(target_type, &relation.name),
    };

    Ok(accessor)
}

fn direct_relation_cardinality(target_property: &Property) -> Cardinality {
    match target_property.unique {
        true => Cardinality::One,
        false => Cardinality::Many,
    }
}

fn direct_relation_presence(relation: &DirectRelation, source: &Property) -> Presence {
    if Presence::Optional == source.presence {
        return Presence::Optional;
    }

    relation.presence
}

fn build_direct_relation_to_one(
    relation: &Relation,
    spec: &DirectRelation,
    source_property: &Property,
    target_type: &Type,
) -> Field {
    Field::new(
        relation.name.clone(),
        match direct_relation_presence(spec, source_property) {
            Presence::Required => TypeRef::named_nn(target_type.name.clone()),
            Presence::Optional => TypeRef::named(target_type.name.clone()),
        },
        default_resolver,
    )
}

fn build_mediated_relation(
    types: &HashMap<&TypeName, &Type>,
    relation: &Relation,
) -> Result<Field, SchemaError> {
    let target_type = get_target_type(relation, types)?;
    Ok(build_accessor_by_filter(target_type, &relation.name))
}

fn get_target_type<'n, 't: 'n>(
    relation: &'n Relation,
    types: &'t HashMap<&'t TypeName, &'t Type>,
) -> Result<&'t Type, SchemaError> {
    let Some(target_type) = types.get(&relation.target).copied() else {
        return Err(SchemaError(format!(
            "Relation [{}] refers to unknow type [{}]",
            relation.name, relation.target
        )));
    };

    Ok(target_type)
}

// fn build_reverse_relation(relation: &ReverseRelation) -> Field {
//     todo!()
// }

fn build_connection_for_type(ty: &Type) -> Object {
    let mut obj = Object::new(connection_name_for_type(ty));

    let page_info = Field::new(
        "page_info",
        TypeRef::named_nn(TYPE_NAME_PAGE_INFO),
        default_resolver,
    );

    let edges = Field::new(
        "edges",
        TypeRef::named_nn_list(edge_name_for_type(ty)),
        default_resolver,
    );

    obj = obj.field(page_info);
    obj = obj.field(edges);

    obj
}

fn build_connection_edge_for_type(ty: &Type) -> Object {
    let mut obj = Object::new(connection_name_for_type(ty));

    let meta = Field::new(
        "metadata",
        TypeRef::named_nn(TYPE_NAME_METADATA),
        default_resolver,
    );

    let node = Field::new("node", TypeRef::named_nn(ty.name.clone()), default_resolver);

    obj = obj.field(meta);
    obj = obj.field(node);

    obj
}

fn connection_name_for_type(ty: &Type) -> String {
    format!("{}_connection", ty.name)
}

fn edge_name_for_type(ty: &Type) -> String {
    format!("{}_edge", ty.name)
}

fn filter_input_name_for_type(ty: &TypeName) -> String {
    format!("{}_filter", ty)
}

fn get_type_ref(kind: Kind, presence: Presence, cardinality: Cardinality) -> TypeRef {
    let type_name = match kind {
        Kind::Integer => TypeRef::INT,
        Kind::Float => TypeRef::FLOAT,
        Kind::String => TypeRef::STRING,
        Kind::Boolean => TypeRef::BOOLEAN,
        Kind::Uuid | Kind::DateTime => TypeRef::STRING, // GraphQL doesn't have native UUID/DateTime by default
    };

    let type_ref = match cardinality {
        Cardinality::Many => match presence {
            Presence::Required => TypeRef::named_nn_list_nn(type_name),
            Presence::Optional => TypeRef::named_nn_list(type_name),
        },
        Cardinality::One => match presence {
            Presence::Required => TypeRef::named_nn(type_name),
            Presence::Optional => TypeRef::named(type_name),
        },
    };

    type_ref
}
