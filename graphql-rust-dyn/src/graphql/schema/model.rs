use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::Deref;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Type {
    pub name: TypeName,
    pub desc: Option<String>,
    pub table_name: TableName,
    pub key: Vec<PropertyName>,
    pub properties: Vec<Property>,
    pub relations: Vec<Relation>,
    pub reverse_relations: Vec<ReverseRelation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Property {
    pub name: PropertyName,
    pub kind: Kind,
    pub presence: Presence,
    pub cardinality: Cardinality,
    pub unique: bool,
    pub desc: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Relation {
    pub name: PropertyName,
    pub target: TypeName,

    #[serde(flatten)]
    pub kind: RelationType,
    pub desc: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReverseRelation {
    pub name: PropertyName,
    pub source: TypeName,
    pub source_property: PropertyName,
    pub desc: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum RelationType {
    Direct(DirectRelation),
    Mediated(MediatedRelation),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DirectRelation {
    pub source_property: PropertyName,
    pub target_property: PropertyName,
    pub presence: Presence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MediatedRelation {
    pub through: TableName,
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", transparent)]
pub struct TypeName(pub String);

impl TypeName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for TypeName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeName> for String {
    fn from(t: TypeName) -> String {
        t.0
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", transparent)]
pub struct PropertyName(pub String);

impl PropertyName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for PropertyName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PropertyName> for String {
    fn from(t: PropertyName) -> String {
        t.0
    }
}

impl Display for PropertyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for PropertyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", transparent)]
pub struct TableName(pub String);

impl TableName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for TableName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TableName> for String {
    fn from(t: TableName) -> String {
        t.0
    }
}

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Integer,
    Float,
    String,
    Boolean,
    DateTime,
    Uuid,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Presence {
    Required,
    Optional,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cardinality {
    One,
    Many,
}

#[cfg(test)]
mod tests {
    use crate::graphql::schema::model::{
        Cardinality, DirectRelation, Kind, MediatedRelation, Presence, Property, PropertyName,
        Relation, RelationType, ReverseRelation, TableName, Type, TypeName,
    };

    #[test]
    fn test_serialization() {
        let ty = Type {
            name: TypeName("demo".to_string()),
            desc: Some("This is a demo type for testing".to_owned()),
            table_name: TableName::new("t_demo"),
            key: vec![PropertyName::new("id")],
            properties: vec![Property {
                name: PropertyName::new("id"),
                kind: Kind::Uuid,
                presence: Presence::Required,
                cardinality: Cardinality::One,
                unique: true,
                desc: Some("The ID of the instance".to_owned()),
            }],

            relations: vec![
                Relation {
                    name: PropertyName("owner".to_owned()),
                    target: TypeName("Employee".to_owned()),
                    kind: RelationType::Direct(DirectRelation {
                        source_property: PropertyName("owner_id".to_owned()),
                        target_property: PropertyName("id".to_owned()),
                        presence: Presence::Optional,
                    }),
                    desc: Some("The owner ID".to_owned()),
                },
                Relation {
                    name: PropertyName("participants".to_owned()),
                    target: TypeName("Employee".to_owned()),
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
        };

        println!("{}", serde_json::to_string_pretty(&ty).unwrap());
    }
}
