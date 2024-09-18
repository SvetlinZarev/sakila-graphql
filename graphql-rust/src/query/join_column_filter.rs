use crate::query::table_filter::TableFilter;
use crate::query::visitor::Visitor;

#[derive(Debug, Clone)]
pub struct JoinColumnFilter<'f> {
    filter: TableFilter<'f>,
    parent_column: &'f str,
    child_column: &'f str,
}

impl<'f> JoinColumnFilter<'f> {
    pub fn new(filter: TableFilter<'f>, parent_column: &'f str, child_column: &'f str) -> Self {
        Self {
            filter,
            parent_column,
            child_column,
        }
    }

    pub fn accept(&self, v: &mut dyn Visitor<'f>) {
        v.on_join_column_filter(self);
    }

    pub fn parent_column(&self) -> &'f str {
        &self.parent_column
    }

    pub fn child_column(&self) -> &'f str {
        &self.child_column
    }

    pub fn filter(&self) -> &TableFilter<'f> {
        &self.filter
    }
}
