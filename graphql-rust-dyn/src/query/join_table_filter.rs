use crate::query::visitor::Visitor;
use crate::query::TableFilter;

#[derive(Debug, Clone)]
pub struct JoinTableFilter<'f> {
    child_table_filter: TableFilter<'f>,
    join_table: &'f str,

    parent_table_join_col: &'f str,
    join_table_parent_col: &'f str,

    child_table_join_col: &'f str,
    join_table_child_col: &'f str,
}

impl<'f> JoinTableFilter<'f> {
    pub fn new(
        child_table_filter: TableFilter<'f>,
        join_table: &'f str,
        parent_table_join_col: &'f str,
        join_table_parent_col: &'f str,
        child_table_join_col: &'f str,
        join_table_child_col: &'f str,
    ) -> Self {
        Self {
            child_table_filter,
            join_table,
            parent_table_join_col,
            join_table_parent_col,
            child_table_join_col,
            join_table_child_col,
        }
    }

    pub fn accept(&self, v: &mut dyn Visitor<'f>) {
        v.on_join_table_filter(self);
    }

    pub fn filter(&self) -> &TableFilter<'f> {
        &self.child_table_filter
    }

    pub fn join_table(&self) -> &'f str {
        self.join_table
    }

    pub fn parent_table_join_col(&self) -> &'f str {
        self.parent_table_join_col
    }

    pub fn join_table_parent_col(&self) -> &'f str {
        self.join_table_parent_col
    }

    pub fn child_table_join_col(&self) -> &'f str {
        self.child_table_join_col
    }

    pub fn join_table_child_col(&self) -> &'f str {
        self.join_table_child_col
    }
}
