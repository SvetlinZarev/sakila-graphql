use crate::graphql::core::filter::{
    and_filters, contains, eq, gt, gte, is_in, is_not_in, join_column, join_table, lt, lte, neq,
    or_filters, InputFilter, TypeInfo,
};
use crate::graphql::core::loader::{FromRow, WithId};
use crate::graphql::core::query::query;
use crate::graphql::loader::{
    ActorLoader, CategoryLoader, FilmActorIdLoader, FilmCategoryIdLoader, LanguageLoader,
};
use crate::graphql::model::join_tables::{
    JOIN_TABLE__FILM_ACTOR, JOIN_TABLE__FILM_ACTOR__ACTOR_ID, JOIN_TABLE__FILM_ACTOR__FILM_ID,
    JOIN_TABLE__FILM_CATEGORY, JOIN_TABLE__FILM_CATEGORY__CATEGORY_ID,
    JOIN_TABLE__FILM_CATEGORY__FILM_ID,
};
use crate::graphql::model::{
    Actor, ActorFilter, Category, CategoryFilter, Language, LanguageFilter,
};
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
pub struct Film {
    #[graphql(skip)]
    pub film_id: i32,

    #[graphql(skip)]
    pub language_id: i32,

    #[graphql(skip)]
    pub original_language_id: Option<i32>,

    pub title: String,
    pub description: String,
    pub length: i16,
}

#[ComplexObject]
impl Film {
    pub const COLUMN_TITLE: &'static str = "title";
    pub const COLUMN_LENGTH: &'static str = "length";
    pub const COLUMN_LANG_ID: &'static str = "language_id";
    pub const COLUMN_DESCRIPTION: &'static str = "description";
    pub const COLUMN_FILM_ID: &'static str = "film_id";
    pub const COLUMN_LANGUAGE_ID: &'static str = "language_id";
    pub const COLUMN_ORIG_LANG_ID: &'static str = "original_language_id";

    async fn actors<'a>(
        &self,
        ctx: &Context<'a>,
        filter: Option<ActorFilter>,
    ) -> async_graphql::Result<Vec<Actor>> {
        if filter.is_none() {
            let ids = ctx
                .data_unchecked::<DataLoader<FilmActorIdLoader, HashMapCache<FxBuildHasher>>>()
                .load_one(self.film_id)
                .await?;

            let Some(ids) = ids else {
                return Ok(vec![]);
            };

            let actors = ctx
                .data_unchecked::<DataLoader<ActorLoader, HashMapCache<FxBuildHasher>>>()
                .load_many(ids)
                .await?;

            return Ok(actors.into_values().collect());
        }

        let joined_table = JoinedTable {
            join_table: JOIN_TABLE__FILM_ACTOR,
            join_table_join_col: JOIN_TABLE__FILM_ACTOR__ACTOR_ID,
            data_table_join_col: Actor::COLUMN_ACTOR_ID,
            join_table_filter_col: Self::COLUMN_FILM_ID,
            join_table_filter_col_val: &self.film_id,
        };

        Ok(query(ctx, &filter, Some(joined_table)).await?)
    }

    async fn categories<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Vec<Category>> {
        let ids = ctx
            .data_unchecked::<DataLoader<FilmCategoryIdLoader, HashMapCache<FxBuildHasher>>>()
            .load_one(self.film_id)
            .await?;

        let Some(ids) = ids else {
            return Ok(vec![]);
        };

        let loader =
            ctx.data_unchecked::<DataLoader<CategoryLoader, HashMapCache<FxBuildHasher>>>();
        let categories = loader.load_many(ids).await?;

        Ok(categories.into_values().collect())
    }

    async fn language<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Language> {
        let language = ctx
            .data_unchecked::<DataLoader<LanguageLoader, HashMapCache<FxBuildHasher>>>()
            .load_one(self.language_id)
            .await?;

        Ok(language.unwrap_or_else(|| Language::default()))
    }

    async fn original_language<'a>(
        &self,
        ctx: &Context<'a>,
    ) -> async_graphql::Result<Option<Language>> {
        let Some(lang_id) = self.original_language_id else {
            return Ok(None);
        };

        let language = ctx
            .data_unchecked::<DataLoader<LanguageLoader, HashMapCache<FxBuildHasher>>>()
            .load_one(lang_id)
            .await?;

        Ok(language)
    }
}

impl TypeInfo for Film {
    const QUERY_FIELD_TO_DB_COLUMN_MAP: LazyLock<FxHashMap<&'static str, &'static str>> =
        LazyLock::new(|| {
            let mut map = FxHashMap::default();
            map.insert("title", Self::COLUMN_TITLE);
            map.insert("description", Self::COLUMN_DESCRIPTION);
            map.insert("length", Self::COLUMN_LENGTH);
            map.insert("actors", Self::COLUMN_FILM_ID);
            map.insert("categories", Self::COLUMN_FILM_ID);
            map.insert("language", Self::COLUMN_LANGUAGE_ID);
            map.insert("originalLanguage", Self::COLUMN_ORIG_LANG_ID);
            map
        });
}

impl WithId<i32> for Film {
    fn id(&self) -> &i32 {
        &self.film_id
    }
}

impl FromRow for Film {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut result = Self::default();
        for col in row.columns() {
            match col.name() {
                Self::COLUMN_TITLE => result.title = row.try_get(Self::COLUMN_TITLE)?,
                Self::COLUMN_DESCRIPTION => {
                    result.description = row.try_get(Self::COLUMN_DESCRIPTION)?
                }
                Self::COLUMN_LENGTH => result.length = row.try_get(Self::COLUMN_LENGTH)?,
                Self::COLUMN_FILM_ID => result.film_id = row.try_get(Self::COLUMN_FILM_ID)?,
                Self::COLUMN_LANG_ID => result.language_id = row.try_get(Self::COLUMN_LANG_ID)?,
                Self::COLUMN_ORIG_LANG_ID => {
                    result.original_language_id = row.try_get(Self::COLUMN_ORIG_LANG_ID)?
                }
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
pub struct FilmFilter {
    pub and: Option<Vec<FilmFilter>>,
    pub or: Option<Vec<FilmFilter>>,

    pub actor: Option<Box<ActorFilter>>,
    pub category: Option<Box<CategoryFilter>>,
    pub language: Option<Box<LanguageFilter>>,
    pub original_language: Option<Box<LanguageFilter>>,

    pub title_eq: Option<String>,
    pub title_not_eq: Option<String>,
    pub title_in: Option<Vec<String>>,
    pub title_not_in: Option<Vec<String>>,
    pub title_contains: Option<String>,

    pub length_eq: Option<i16>,
    pub length_gt: Option<i16>,
    pub length_gte: Option<i16>,
    pub length_lt: Option<i16>,
    pub length_lte: Option<i16>,
}

impl InputFilter for FilmFilter {
    const TABLE_NAME: &'static str = "film";

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

        eq(&mut g, Film::COLUMN_TITLE, &self.title_eq);
        neq(&mut g, Film::COLUMN_TITLE, &self.title_not_eq);
        is_in(&mut g, Film::COLUMN_TITLE, &self.title_in);
        is_not_in(&mut g, Film::COLUMN_TITLE, &self.title_not_in);
        contains(&mut g, Film::COLUMN_TITLE, &self.title_contains);

        eq(&mut g, Film::COLUMN_LENGTH, &self.length_eq);
        lt(&mut g, Film::COLUMN_LENGTH, &self.length_lt);
        lte(&mut g, Film::COLUMN_LENGTH, &self.length_lte);
        gt(&mut g, Film::COLUMN_LENGTH, &self.length_gt);
        gte(&mut g, Film::COLUMN_LENGTH, &self.length_gte);

        join_column(
            &mut g,
            &self.language,
            Film::COLUMN_LANG_ID,
            Language::LANGUAGE_ID,
        );
        join_column(
            &mut g,
            &self.original_language,
            Film::COLUMN_ORIG_LANG_ID,
            Language::LANGUAGE_ID,
        );

        join_table(
            &mut g,
            &self.category,
            JOIN_TABLE__FILM_CATEGORY,
            JOIN_TABLE__FILM_CATEGORY__FILM_ID,
            JOIN_TABLE__FILM_CATEGORY__CATEGORY_ID,
            Film::COLUMN_FILM_ID,
            Category::COLUMN_CATEGORY_ID,
        );

        join_table(
            &mut g,
            &self.actor,
            JOIN_TABLE__FILM_ACTOR,
            JOIN_TABLE__FILM_ACTOR__FILM_ID,
            JOIN_TABLE__FILM_ACTOR__ACTOR_ID,
            Film::COLUMN_FILM_ID,
            Actor::COLUMN_ACTOR_ID,
        );

        if let MaybeOwned::Owned(g) = g {
            collector.add_group(g);
        }
    }
}
