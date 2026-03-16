use crate::graphql::core::filter::{
    and_filters, eq, is_in, is_not_in, join_table, neq, or_filters, InputFilter, TypeInfo,
};
use crate::graphql::core::loader::{FromRow, WithId};
use crate::graphql::core::query::query;
use crate::graphql::loader::{ActorFilmIdLoader, FilmLoader};
use crate::graphql::model::join_tables::{
    JOIN_TABLE__FILM_ACTOR, JOIN_TABLE__FILM_ACTOR__ACTOR_ID, JOIN_TABLE__FILM_ACTOR__FILM_ID,
};
use crate::graphql::model::{Film, FilmFilter};
use crate::query::{Combinator, FilterGroup, JoinedTable};
use crate::util::MaybeOwned;
use anyhow::Error;
use async_graphql::dataloader::{DataLoader, HashMapCache};
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::sync::LazyLock;
use tokio_postgres::Row;

#[derive(Debug, Clone, Default, SimpleObject)]
#[graphql(complex)]
pub struct Actor {
    #[graphql(skip)]
    pub actor_id: i32,

    pub first_name: String,
    pub last_name: String,
}

#[ComplexObject]
impl Actor {
    pub const COLUMN_FIRST_NAME: &'static str = "first_name";
    pub const COLUMN_LAST_NAME: &'static str = "last_name";
    pub const COLUMN_ACTOR_ID: &'static str = "actor_id";

    async fn films<'a>(
        &self,
        ctx: &Context<'a>,
        filter: Option<FilmFilter>,
    ) -> async_graphql::Result<Vec<Film>> {
        if filter.is_none() {
            let ids = ctx
                .data_unchecked::<DataLoader<ActorFilmIdLoader, HashMapCache<FxBuildHasher>>>()
                .load_one(self.actor_id)
                .await?;

            let Some(ids) = ids else {
                return Ok(vec![]);
            };

            let films = ctx
                .data_unchecked::<DataLoader<FilmLoader, HashMapCache<FxBuildHasher>>>()
                .load_many(ids)
                .await?;

            return Ok(films.into_values().collect());
        }

        let joined_table = JoinedTable {
            join_table: JOIN_TABLE__FILM_ACTOR,
            join_table_join_col: JOIN_TABLE__FILM_ACTOR__FILM_ID,
            data_table_join_col: Film::COLUMN_FILM_ID,
            join_table_filter_col: Self::COLUMN_ACTOR_ID,
            join_table_filter_col_val: &self.actor_id,
        };

        Ok(query(ctx, &filter, Some(joined_table)).await?)
    }
}

impl WithId<i32> for Actor {
    fn id(&self) -> &i32 {
        &self.actor_id
    }
}

impl TypeInfo for Actor {
    const QUERY_FIELD_TO_DB_COLUMN_MAP: LazyLock<FxHashMap<&'static str, &'static str>> =
        LazyLock::new(|| {
            let mut map = FxHashMap::default();
            map.insert("firstName", Self::COLUMN_FIRST_NAME);
            map.insert("lastName", Self::COLUMN_LAST_NAME);
            map.insert("films", Self::COLUMN_ACTOR_ID);
            map
        });
}

impl FromRow for Actor {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut result = Self::default();
        for col in row.columns() {
            match col.name() {
                Self::COLUMN_ACTOR_ID => result.actor_id = row.try_get(Self::COLUMN_ACTOR_ID)?,
                Self::COLUMN_FIRST_NAME => {
                    result.first_name = row.try_get(Self::COLUMN_FIRST_NAME)?
                }
                Self::COLUMN_LAST_NAME => result.last_name = row.try_get(Self::COLUMN_LAST_NAME)?,
                col => tracing::debug!(
                    "fetched unknown column '{}' for type '{}'",
                    col,
                    std::any::type_name::<Self>()
                ),
            }
        }

        Ok(result)
    }
}

#[derive(Debug, Clone, Default, InputObject)]
pub struct ActorFilter {
    pub and: Option<Vec<ActorFilter>>,
    pub or: Option<Vec<ActorFilter>>,

    pub film: Option<Box<FilmFilter>>,

    pub first_name_eq: Option<String>,
    pub first_name_in: Option<Vec<String>>,

    pub first_name_not_eq: Option<String>,
    pub first_name_not_in: Option<Vec<String>>,

    pub last_name_eq: Option<String>,
    pub last_name_in: Option<Vec<String>>,

    pub last_name_not_eq: Option<String>,
    pub last_name_not_in: Option<Vec<String>>,
}

impl InputFilter for ActorFilter {
    const TABLE_NAME: &'static str = "actor";

    fn or_filters(&self) -> Option<&[Self]>
    where
        Self: Sized,
    {
        self.or.as_ref().map(|x| x.as_slice())
    }

    fn and_filters(&self) -> Option<&[Self]>
    where
        Self: Sized,
    {
        self.and.as_ref().map(|x| x.as_slice())
    }

    fn collect_into<'f>(&'f self, collector: &mut FilterGroup<'f>) {
        let mut g = match collector.combinator() == Combinator::And {
            true => MaybeOwned::Borrowed(collector),
            false => MaybeOwned::Owned(FilterGroup::new(Combinator::And)),
        };

        and_filters(&mut g, self);
        or_filters(&mut g, self);

        eq(&mut g, Actor::COLUMN_FIRST_NAME, &self.first_name_eq);
        neq(&mut g, Actor::COLUMN_FIRST_NAME, &self.first_name_not_eq);

        is_in(&mut g, Actor::COLUMN_FIRST_NAME, &self.first_name_in);
        is_not_in(&mut g, Actor::COLUMN_FIRST_NAME, &self.first_name_not_in);

        eq(&mut g, Actor::COLUMN_LAST_NAME, &self.last_name_eq);
        neq(&mut g, Actor::COLUMN_LAST_NAME, &self.last_name_not_eq);

        is_in(&mut g, Actor::COLUMN_LAST_NAME, &self.last_name_in);
        is_not_in(&mut g, Actor::COLUMN_LAST_NAME, &self.last_name_not_in);

        join_table(
            &mut g,
            &self.film,
            JOIN_TABLE__FILM_ACTOR,
            JOIN_TABLE__FILM_ACTOR__ACTOR_ID,
            JOIN_TABLE__FILM_ACTOR__FILM_ID,
            Actor::COLUMN_ACTOR_ID,
            Film::COLUMN_FILM_ID,
        );

        if let MaybeOwned::Owned(g) = g {
            collector.add_group(g);
        }
    }
}
