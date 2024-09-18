use tokio_postgres::types::ToSql;

#[derive(Debug, Clone)]
pub struct JoinedTable<'f> {
    pub join_table: &'f str,
    pub join_table_join_col: &'f str,
    pub data_table_join_col: &'f str,
    pub join_table_filter_col: &'f str,
    pub join_table_filter_col_val: &'f (dyn ToSql + Sync),
}
