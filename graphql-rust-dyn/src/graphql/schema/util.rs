use crate::util::Either;
use async_graphql::dynamic::{Field, Object, SchemaBuilder, Type};

pub struct SubGraph {
    pub objects: Either<Type, Vec<Type>>,
    pub fields: Either<Field, Vec<Field>>,
}

impl SubGraph {
    pub fn new() -> Self {
        SubGraph {
            objects: Either::Right(vec![]),
            fields: Either::Right(vec![]),
        }
    }

    pub fn add_object(&mut self, object: impl Into<Type>) {
        self.add(object.into(), |s| &mut s.objects)
    }

    pub fn add_field(&mut self, field: Field) {
        self.add(field, |s| &mut s.fields)
    }

    fn add<'a, T: 'a>(
        &'a mut self,
        val: T,
        mut selector: impl FnMut(&'a mut Self) -> &'a mut Either<T, Vec<T>>,
    ) {
        let mut values = Either::Right(vec![]);
        let selected = selector(self);

        std::mem::swap(selected, &mut values);

        values = match values {
            Either::Left(v) => Either::Right(vec![v, val]),
            Either::Right(mut vals) => match vals.is_empty() {
                true => Either::Left(val),
                false => {
                    vals.push(val);
                    Either::Right(vals)
                }
            },
        };

        std::mem::swap(selected, &mut values);
    }

    #[must_use]
    pub fn register_objects(&mut self, builder: SchemaBuilder) -> SchemaBuilder {
        let mut values = Either::Right(vec![]);
        std::mem::swap(&mut self.objects, &mut values);

        match values {
            Either::Left(object) => builder.register(object),
            Either::Right(objects) => objects.into_iter().fold(builder, |b, o| b.register(o)),
        }
    }

    #[must_use]
    pub fn register_fields(&mut self, object: Object) -> Object {
        let mut values = Either::Right(vec![]);
        std::mem::swap(&mut self.fields, &mut values);

        match values {
            Either::Left(field) => object.field(field),
            Either::Right(fields) => fields.into_iter().fold(object, |obj, f| obj.field(f)),
        }
    }
}
