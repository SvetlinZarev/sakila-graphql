use crate::query::join_column_filter::JoinColumnFilter;
use crate::query::join_table_filter::JoinTableFilter;
use crate::query::value_filter::ValueFilter;
use crate::query::visitor::Visitor;

#[derive(Debug, Clone)]
pub enum FilterType<'f> {
    ValueFilter(ValueFilter<'f>),
    JoinColumnFilter(JoinColumnFilter<'f>),
    JoinTableFilter(JoinTableFilter<'f>),
}

impl<'f> FilterType<'f> {
    pub fn accept(&self, v: &mut dyn Visitor<'f>) {
        v.on_filter_type(self);
    }
}

impl<'f> From<ValueFilter<'f>> for FilterType<'f> {
    fn from(value: ValueFilter<'f>) -> Self {
        FilterType::ValueFilter(value)
    }
}

impl<'f> From<JoinColumnFilter<'f>> for FilterType<'f> {
    fn from(value: JoinColumnFilter<'f>) -> Self {
        FilterType::JoinColumnFilter(value)
    }
}

impl<'f> From<JoinTableFilter<'f>> for FilterType<'f> {
    fn from(value: JoinTableFilter<'f>) -> Self {
        FilterType::JoinTableFilter(value)
    }
}
