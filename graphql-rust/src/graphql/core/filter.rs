use crate::query::{
    Combinator, FilterGroup, JoinColumnFilter, JoinTableFilter, Operation, TableFilter, ValueFilter,
};
use crate::util::MaybeOwned;
use rustc_hash::FxHashMap;
use std::sync::LazyLock;
use tokio_postgres::types::ToSql;

pub trait InputFilter {
    const TABLE_NAME: &'static str;

    fn or_filters(&self) -> Option<&[Self]>
    where
        Self: Sized;

    fn and_filters(&self) -> Option<&[Self]>
    where
        Self: Sized;

    fn collect_into<'f>(&'f self, collector: &mut FilterGroup<'f>);
}

pub trait TypeInfo {
    const QUERY_FIELD_TO_DB_COLUMN_MAP: LazyLock<FxHashMap<&'static str, &'static str>>;
}

pub fn and_filters<'f, T: InputFilter>(g: &mut FilterGroup<'f>, f: &'f T) {
    if let Some(filters) = f.and_filters() {
        let mut c = match g.combinator() == Combinator::And {
            true => MaybeOwned::Borrowed(g),
            false => MaybeOwned::Owned(FilterGroup::new(Combinator::And)),
        };

        for filter in filters {
            filter.collect_into(&mut c);
        }

        if let MaybeOwned::Owned(x) = c {
            g.add_group(x);
        }
    }
}

pub fn or_filters<'f, T: InputFilter>(g: &mut FilterGroup<'f>, f: &'f T) {
    if let Some(filters) = f.or_filters() {
        let mut c = match g.combinator() == Combinator::Or {
            true => MaybeOwned::Borrowed(g),
            false => MaybeOwned::Owned(FilterGroup::new(Combinator::Or)),
        };

        for filter in filters {
            filter.collect_into(&mut c);
        }

        if let MaybeOwned::Owned(x) = c {
            g.add_group(x);
        }
    }
}

fn op<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, operation: Operation, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    if let Some(value) = value.as_ref() {
        let f = ValueFilter::new(operation, column, value);
        g.add_filter(f);
    }
}

#[allow(unused)]
pub fn eq<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Eq, value)
}

#[allow(unused)]
pub fn neq<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Neq, value)
}

#[allow(unused)]
pub fn lt<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Lt, value)
}

#[allow(unused)]
pub fn lte<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Lte, value)
}

#[allow(unused)]
pub fn gt<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Gt, value)
}

#[allow(unused)]
pub fn gte<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Gte, value)
}

#[allow(unused)]
pub fn is_in<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::In, value)
}

#[allow(unused)]
pub fn is_not_in<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::NotIn, value)
}

#[allow(unused)]
pub fn is_null<'a>(g: &mut FilterGroup<'a>, column: &'a str) {
    op(g, column, Operation::IsNull, &Some(""))
}

#[allow(unused)]
pub fn is_not_null<'a>(g: &mut FilterGroup<'a>, column: &'a str) {
    op(g, column, Operation::IsNotNull, &Some(""))
}

#[allow(unused)]
pub fn contains<'a, T>(g: &mut FilterGroup<'a>, column: &'a str, value: &'a Option<T>)
where
    T: 'a + ToSql + Sync,
{
    op(g, column, Operation::Contains, value)
}

#[allow(unused)]
pub fn join_column<'a, C>(
    g: &mut FilterGroup<'a>,
    child_filter: &'a Option<Box<C>>,
    parent_col: &'a str,
    child_col: &'a str,
) where
    C: InputFilter,
{
    if let Some(child_filter) = child_filter.as_ref() {
        let mut tf = TableFilter::new(C::TABLE_NAME);
        child_filter.collect_into(tf.filter_group_mut());

        let jf = JoinColumnFilter::new(tf, parent_col, child_col);
        g.add_filter(jf);
    }
}

#[allow(unused)]
pub fn join_table<'a, C>(
    g: &mut FilterGroup<'a>,
    child_filter: &'a Option<Box<C>>,
    join_table: &'a str,
    jt_parent_col: &'a str,
    jt_child_col: &'a str,
    parent_join_col: &'a str,
    child_join_col: &'a str,
) where
    C: InputFilter,
{
    if let Some(child_filter) = child_filter.as_ref() {
        let mut tf = TableFilter::new(C::TABLE_NAME);
        child_filter.collect_into(tf.filter_group_mut());

        let jf = JoinTableFilter::new(
            tf,
            join_table,
            parent_join_col,
            jt_parent_col,
            child_join_col,
            jt_child_col,
        );
        g.add_filter(jf);
    }
}
