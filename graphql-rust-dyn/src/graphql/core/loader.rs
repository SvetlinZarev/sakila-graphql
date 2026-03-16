use anyhow::Error;
use async_graphql::futures_util::StreamExt;
use deadpool_postgres::{GenericClient, Pool};
use std::collections::HashMap;
use std::hash::Hash;
use std::pin::pin;
use tokio_postgres::types::{FromSqlOwned, ToSql};
use tokio_postgres::Row;

pub trait FromRow {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized;
}

impl FromRow for i32 {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(row.try_get("__value")?)
    }
}

pub trait WithId<ID> {
    fn id(&self) -> &ID;
}

pub async fn load<T: FromRow>(
    pool: &Pool,
    query: String,
    params: Vec<&(dyn ToSql + Sync)>,
) -> Result<Vec<T>, anyhow::Error> {
    let con = pool.get().await?;
    let stm = con.prepare_cached(&query).await?;

    let rows = con.query_raw(&stm, params).await?;
    let mut rows = pin!(rows);

    let mut result = vec![];
    while let Some(row) = rows.next().await {
        let row = row?;
        let value = T::from_row(&row)?;
        result.push(value);
    }

    Ok(result)
}

pub async fn load_one_by_key<I, T, K>(
    pool: &Pool,
    query: &str,
    keys: &[K],
) -> anyhow::Result<HashMap<I, T>>
where
    I: Clone + Eq + Hash,
    T: WithId<I> + FromRow,
    K: ToSql + Sync,
{
    let con = pool.get().await?;
    let stm = con.prepare_cached(&query).await?;

    let rows = con.query_raw(&stm, &[keys]).await?;
    let mut rows = pin!(rows);

    let mut result = HashMap::new();

    while let Some(row) = rows.next().await {
        let row = row?;
        let value = T::from_row(&row)?;
        result.insert(value.id().to_owned(), value);
    }

    Ok(result)
}

pub const KEY_BULK_LOAD: &str = "__loader_key";

pub async fn load_many_by_key<T, KEYS, ID>(
    pool: &Pool,
    query: &str,
    keys: &[KEYS],
) -> anyhow::Result<HashMap<ID, Vec<T>>>
where
    T: FromRow,
    KEYS: ToSql + Sync,
    ID: FromSqlOwned + Sync + Clone + Eq + Hash + 'static,
{
    let con = pool.get().await?;
    let stm = con.prepare_cached(&query).await?;

    let rows = con.query_raw(&stm, &[keys]).await?;
    let mut rows = pin!(rows);

    let mut result = HashMap::new();

    while let Some(row) = rows.next().await {
        let row = row?;

        let key: ID = row.try_get(KEY_BULK_LOAD)?;
        let value = T::from_row(&row)?;

        result.entry(key).or_insert_with(|| vec![]).push(value);
    }

    Ok(result)
}
