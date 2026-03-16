use crate::query::filter_group::FilterGroup;
use crate::query::ops::Combinator;
use crate::query::visitor::Visitor;

#[derive(Debug, Clone)]
pub struct TableFilter<'f> {
    table: &'f str,
    filters: FilterGroup<'f>,
}

impl<'f> TableFilter<'f> {
    pub fn new(table: &'f str) -> Self {
        Self {
            table,
            filters: FilterGroup::new(Combinator::And),
        }
    }

    pub fn accept(&self, v: &mut dyn Visitor<'f>) {
        v.on_table_filter(self);
    }

    pub fn table_name(&self) -> &'f str {
        self.table
    }

    pub fn filter_group(&self) -> &FilterGroup<'f> {
        &self.filters
    }

    pub fn filter_group_mut(&mut self) -> &mut FilterGroup<'f> {
        &mut self.filters
    }
}
