use crate::query::filter_group::FilterGroup;
use crate::query::filter_type::FilterType;
use crate::query::join_column_filter::JoinColumnFilter;
use crate::query::join_table_filter::JoinTableFilter;
use crate::query::table_filter::TableFilter;
use crate::query::value_filter::ValueFilter;

pub trait Visitor<'f> {
    fn on_filter_group(&mut self, f: &FilterGroup<'f>);
    fn on_filter_type(&mut self, f: &FilterType<'f>);
    fn on_value_filter(&mut self, f: &ValueFilter<'f>);
    fn on_table_filter(&mut self, f: &TableFilter<'f>);
    fn on_join_column_filter(&mut self, f: &JoinColumnFilter<'f>);
    fn on_join_table_filter(&mut self, f: &JoinTableFilter<'f>);
}
