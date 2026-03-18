use crate::graphql::schema::model::{
    Cardinality, DirectRelation, Kind, Presence, Property, Relation, RelationType, ReverseRelation,
    Type, TypeName,
};
use async_graphql::dynamic::*;
use foldhash::{HashMap, HashMapExt};
use serde_json::Value;

pub fn build_graphql_schema(metamodel: Vec<Type>) -> Result<Schema, SchemaError> {
    let mut schema_builder = Schema::build("Query", None, None);
    let mut query = Object::new("Query");

    let mut types = HashMap::new();
    for ty in &metamodel {
        types.insert(&ty.name, ty);
    }

    for ty in &metamodel {
        let mut obj = Object::new(ty.name.clone());
        if let Some(desc) = ty.desc.as_ref() {
            obj = obj.description(desc);
        }

        for property in &ty.properties {
            obj = obj.field(build_property(property));
            if property.unique {
                query = query.field(build_accessor_by_key(ty, property));
            }
        }

        for relation in &ty.relations {
            obj = obj.field(build_relation(&types, ty, relation)?);
        }
        //
        // for relation in &ty.reverse_relations {
        //     obj = obj.field(build_reverse_relation(relation));
        // }

        let filter = build_type_filter_input(ty);
        query = query.field(build_accessor_by_filter(ty, &filter));

        schema_builder = schema_builder.register(filter);
        schema_builder = schema_builder.register(obj);
    }

    schema_builder.register(query).finish()
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
    let name = format!("{}__by_{}", &ty.name, &property.name);
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

fn build_accessor_by_filter(ty: &Type, filter: &InputObject) -> Field {
    let name = format!("{}__by_filter", &ty.name);
    let type_ref = TypeRef::named(ty.name.clone());
    let mut field = Field::new(name, type_ref, default_resolver);

    field = field.argument(InputValue::new(
        "filter",
        TypeRef::named(filter.type_name()),
    ));

    field
}

fn build_type_filter_input(ty: &Type) -> InputObject {
    const SUFFIXES_COMPARISON: &[&str] = &["lt", "lte", "gt", "gte", "eq", "neq"];
    const SUFFIX_IS_NULL: &str = "is_null";
    const SUFFIX_CONTAINS: &str = "contains";
    const SUFFIX_NOT_CONTAINS: &str = "not_contains";

    let mut filter = InputObject::new(format!("{}_filter", &ty.name));
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

    filter
}

fn build_relation(
    types: &HashMap<&TypeName, &Type>,
    source: &Type,
    relation: &Relation,
) -> Result<Field, SchemaError> {
    match &relation.kind {
        RelationType::Direct(spec) => build_direct_relation(types, source, relation, spec),
        RelationType::Mediated(_) => todo!(),
    }
}

fn build_direct_relation(
    types: &HashMap<&TypeName, &Type>,
    source_type: &Type,
    relation: &Relation,
    spec: &DirectRelation,
) -> Result<Field, SchemaError> {
    let Some(target_type) = types.get(&relation.target).copied() else {
        return Err(SchemaError(format!(
            "Relation [{}] refers to unknow type [{}]",
            relation.name, relation.target
        )));
    };

    let source_property = source_type
        .properties
        .iter()
        .find(|p| p.name == spec.source_property)
        .ok_or_else(|| SchemaError(format!("Relation {} refers to type {} via source property {}, which is not present in the source type", relation.name, relation.target, spec.source_property)))?;

    let target_property = target_type
        .properties
        .iter()
        .find(|p| p.name == spec.target_property)
        .ok_or_else(|| SchemaError(format!("Relation {} refers to type {} via target property {}, which is not present in the target type", relation.name, relation.target, spec.target_property)))?;

    let field = match direct_relation_cardinality(target_property) {
        Cardinality::One => Field::new(
            relation.name.clone(),
            match direct_relation_presence(spec, source_property) {
                Presence::Required => TypeRef::named_nn(target_type.name.clone()),
                Presence::Optional => TypeRef::named(target_type.name.clone()),
            },
            default_resolver,
        ),

        Cardinality::Many => Field::new(
            relation.name.clone(),
            TypeRef::named_nn_list(target_type.name.clone()),
            default_resolver,
        ),
    };

    Ok(field)
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

fn build_reverse_relation(relation: &ReverseRelation) -> Field {
    todo!()
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

fn default_resolver(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        let field_name = ctx.field().name().to_string();

        // Use downcast_ref directly on the FieldValue reference
        let parent_json = ctx
            .parent_value
            .downcast_ref::<Value>() // This looks for data stored via owned_any/borrowed_any
            .ok_or_else(|| {
                format!(
                    "Field '{}' expected a JSON parent but found none",
                    field_name
                )
            })?;

        // Extract the value from the JSON
        let val = parent_json.get(&field_name).cloned().unwrap_or(Value::Null);

        match val {
            Value::Object(_) => Ok(Some(FieldValue::owned_any(val))),
            Value::Array(arr) => Ok(Some(FieldValue::list(
                arr.into_iter().map(FieldValue::owned_any),
            ))),
            Value::Null => Ok(None),
            _ => {
                // Use the value! macro to convert JSON scalar to GQL scalar
                Ok(Some(FieldValue::value(async_graphql::value!(val))))
            }
        }
    })
}
