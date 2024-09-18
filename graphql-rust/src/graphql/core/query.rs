use crate::graphql::core::filter::{InputFilter, TypeInfo};
use crate::graphql::core::loader::{load, FromRow};
use crate::query::{JoinedTable, SqlVisitor, TableFilter};
use async_graphql::Context;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;

pub async fn query<'c, 'f, T: TypeInfo + FromRow, F: InputFilter>(
    ctx: &Context<'c>,
    filter: &Option<F>,
    joined_table: Option<JoinedTable<'f>>,
) -> anyhow::Result<Vec<T>> {
    let selected = extract_selected_properties::<T>(ctx);
    let (sql, params) = process(&filter, &selected, joined_table);

    let db = ctx.data_unchecked::<Pool>();
    let result = load(db, sql, params).await?;

    Ok(result)
}

fn extract_selected_properties<'a, 's, T: TypeInfo>(ctx: &'s Context<'a>) -> Vec<&'s str> {
    let mut columns = ctx
        .field()
        .selection_set()
        .map(|s| s.name())
        .filter_map(|s| T::QUERY_FIELD_TO_DB_COLUMN_MAP.get(&s).copied())
        .collect::<Vec<_>>();

    columns.sort_unstable();
    columns.dedup();
    columns
}

fn process<'l, F: InputFilter>(
    f: &'l Option<F>,
    s: &'l [&'l str],
    joined_table: Option<JoinedTable<'l>>,
) -> (String, Vec<&'l (dyn ToSql + Sync)>) {
    let mut tf = TableFilter::new(F::TABLE_NAME);
    if let Some(filter) = f.as_ref() {
        filter.collect_into(tf.filter_group_mut());
    }

    let visitor = match joined_table {
        None => SqlVisitor::new(),
        Some(j) => SqlVisitor::with_joined_table(j),
    };

    let (sql, params) = visitor.translate(&tf, s);
    tracing::debug!(
        query=sql,
        paramters=?params,
        table=F::TABLE_NAME,
    );

    (sql, params)
}
