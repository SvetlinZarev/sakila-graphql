use crate::query::ops::Operation;
use crate::query::visitor::Visitor;
use tokio_postgres::types::ToSql;

#[derive(Debug, Clone)]
pub struct ValueFilter<'f> {
    operation: Operation,
    column: &'f str,
    value: &'f (dyn ToSql + Sync),
}

impl<'f> ValueFilter<'f> {
    pub fn new(operation: Operation, column: &'f str, value: &'f (dyn ToSql + Sync)) -> Self {
        Self {
            operation,
            column,
            value,
        }
    }

    pub fn accept(&self, v: &mut dyn Visitor<'f>) {
        v.on_value_filter(self);
    }

    pub fn operation(&self) -> Operation {
        self.operation
    }

    pub fn column(&self) -> &'f str {
        self.column
    }

    pub fn value(&self) -> &'f (dyn ToSql + Sync) {
        self.value
    }
}
