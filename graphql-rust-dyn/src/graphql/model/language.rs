use crate::graphql::core::filter::{
    and_filters, contains, eq, is_in, is_not_in, neq, or_filters, InputFilter, TypeInfo,
};
use crate::graphql::core::loader::{FromRow, WithId};
use crate::query::{Combinator, FilterGroup};
use crate::util::MaybeOwned;
use anyhow::Error;
use async_graphql::{InputObject, SimpleObject};
use rustc_hash::FxHashMap;
use std::sync::LazyLock;
use tokio_postgres::Row;

#[derive(Debug, Clone, Default, SimpleObject)]
pub struct Language {
    #[graphql(skip)]
    pub language_id: i32,

    pub name: String,
}

impl Language {
    pub const COLUMN_NAME: &'static str = "name";
    pub const LANGUAGE_ID: &'static str = "language_id";
}

impl TypeInfo for Language {
    const QUERY_FIELD_TO_DB_COLUMN_MAP: LazyLock<FxHashMap<&'static str, &'static str>> =
        LazyLock::new(|| {
            let mut map = FxHashMap::default();
            map.insert("name", Self::COLUMN_NAME);
            map
        });
}

impl WithId<i32> for Language {
    fn id(&self) -> &i32 {
        &self.language_id
    }
}

impl FromRow for Language {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut result = Self::default();
        for col in row.columns() {
            match col.name() {
                Self::COLUMN_NAME => result.name = row.try_get(Self::COLUMN_NAME)?,
                Self::LANGUAGE_ID => result.language_id = row.try_get(Self::LANGUAGE_ID)?,
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
pub struct LanguageFilter {
    pub name_eq: Option<String>,
    pub name_in: Option<Vec<String>>,

    pub name_not_eq: Option<String>,
    pub name_not_in: Option<Vec<String>>,

    pub name_contains: Option<String>,
}

impl InputFilter for LanguageFilter {
    const TABLE_NAME: &'static str = "language";

    fn or_filters(&self) -> Option<&[Self]>
    where
        Self: Sized,
    {
        None
    }

    fn and_filters(&self) -> Option<&[Self]>
    where
        Self: Sized,
    {
        None
    }

    fn collect_into<'f>(&'f self, collector: &mut FilterGroup<'f>) {
        let mut g = match collector.combinator() == Combinator::And {
            true => MaybeOwned::Borrowed(collector),
            false => MaybeOwned::Owned(FilterGroup::new(Combinator::And)),
        };

        and_filters(&mut g, self);
        or_filters(&mut g, self);

        eq(&mut g, Language::COLUMN_NAME, &self.name_eq);
        neq(&mut g, Language::COLUMN_NAME, &self.name_not_eq);

        is_in(&mut g, Language::COLUMN_NAME, &self.name_in);
        is_not_in(&mut g, Language::COLUMN_NAME, &self.name_not_in);

        contains(&mut g, Language::COLUMN_NAME, &self.name_contains);

        if let MaybeOwned::Owned(g) = g {
            collector.add_group(g);
        }
    }
}
