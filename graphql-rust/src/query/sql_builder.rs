use crate::query::filter_group::FilterGroup;
use crate::query::filter_type::FilterType;
use crate::query::join_column_filter::JoinColumnFilter;
use crate::query::join_table_filter::JoinTableFilter;
use crate::query::joined_table::JoinedTable;
use crate::query::ops::Operation;
use crate::query::table_filter::TableFilter;
use crate::query::value_filter::ValueFilter;
use crate::query::visitor::Visitor;
use std::fmt::Write;
use tokio_postgres::types::ToSql;

const SQL_QUERY_BUFFER_INITIAL_CAPACITY: usize = 1024;

#[derive(Debug, Default)]
pub struct SqlVisitor<'v> {
    joined_table: Option<JoinedTable<'v>>,
    next_table: u32,
    current_table: Vec<u32>,
    query: String,
    params: Vec<&'v (dyn ToSql + Sync)>,
}

impl<'v> SqlVisitor<'v> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_joined_table(joined_table: JoinedTable<'v>) -> Self {
        Self {
            joined_table: Some(joined_table),
            ..Default::default()
        }
    }

    pub fn translate(
        mut self,
        filter: &TableFilter<'v>,
        select: &[&str],
    ) -> (String, Vec<&'v (dyn ToSql + Sync)>) {
        self.query.reserve(SQL_QUERY_BUFFER_INITIAL_CAPACITY);

        let current_table_id = self.push_new_table();

        self.query.push_str("SELECT ");
        if select.is_empty() {
            write!(&mut self.query, "T{}.*", current_table_id).unwrap();
        } else {
            for (idx, sel) in select.iter().enumerate() {
                if idx > 0 {
                    self.query.push_str(", ");
                }
                write!(&mut self.query, "T{}.{}", current_table_id, sel).unwrap();
            }
        }

        self.query.push_str(" FROM ");
        if let Some(j) = self.joined_table.as_ref() {
            write!(
                &mut self.query,
                "{} AS J INNER JOIN {} AS T{} ON J.{} = T{}.{}",
                j.join_table,
                filter.table_name(),
                current_table_id,
                j.join_table_join_col,
                current_table_id,
                j.data_table_join_col
            )
            .unwrap();
        } else {
            write!(
                &mut self.query,
                "{} AS T{}",
                filter.table_name(),
                current_table_id
            )
            .unwrap();
        }

        if !filter.filter_group().is_empty() || self.joined_table.is_some() {
            self.query.push_str(" WHERE ");
        }

        if !filter.filter_group().is_empty() {
            filter.accept(&mut self);
        }

        if let Some(j) = self.joined_table.take() {
            if !filter.filter_group().is_empty() {
                self.query.push_str(" AND ");
            }

            let param = self.add_param(j.join_table_filter_col_val);
            write!(
                &mut self.query,
                "J.{} = ${}",
                j.join_table_filter_col, param
            )
            .unwrap();
        }

        (self.query, self.params)
    }

    fn add_param(&mut self, p: &'v (dyn ToSql + Sync)) -> usize {
        self.params.push(p);
        self.params.len()
    }

    fn push_new_table(&mut self) -> u32 {
        self.current_table.push(self.next_table);
        self.next_table += 1;
        self.next_table - 1
    }

    fn pop_old_table(&mut self) {
        self.current_table.pop();
    }

    fn current_table_id(&self) -> u32 {
        self.current_table.last().copied().unwrap()
    }
}

impl<'v> Visitor<'v> for SqlVisitor<'v> {
    fn on_table_filter(&mut self, f: &TableFilter<'v>) {
        f.filter_group().accept(self);
    }

    fn on_filter_group(&mut self, f: &FilterGroup<'v>) {
        let write_brackets = (f.filters().len() + f.groups().len()) > 1;
        if write_brackets {
            self.query.push_str("(");
        }

        for (idx, filter) in f.filters().iter().enumerate() {
            if idx > 0 {
                write!(&mut self.query, " {} ", f.combinator()).unwrap();
            }

            filter.accept(self);
        }

        for (idx, group) in f.groups().iter().enumerate() {
            if idx > 0 || !f.filters().is_empty() {
                write!(&mut self.query, " {} ", f.combinator()).unwrap();
            }

            group.accept(self);
        }

        if write_brackets {
            self.query.push_str(")");
        }
    }

    fn on_filter_type(&mut self, f: &FilterType<'v>) {
        match f {
            FilterType::ValueFilter(x) => x.accept(self),
            FilterType::JoinColumnFilter(x) => x.accept(self),
            FilterType::JoinTableFilter(x) => x.accept(self),
        }
    }

    fn on_value_filter(&mut self, f: &ValueFilter<'v>) {
        let current_table_id = self.current_table_id();

        write!(&mut self.query, "T{}.{} ", current_table_id, f.column()).unwrap();

        let val = f.value();

        match f.operation() {
            Operation::Eq => {
                let param = self.add_param(val);
                write!(&mut self.query, "= ${}", param)
            }
            Operation::Neq => {
                let param = self.add_param(val);
                write!(&mut self.query, "<> ${}", param)
            }
            Operation::Lt => {
                let param = self.add_param(val);
                write!(&mut self.query, "< ${}", param)
            }
            Operation::Gt => {
                let param = self.add_param(val);
                write!(&mut self.query, "> ${}", param)
            }
            Operation::Lte => {
                let param = self.add_param(val);
                write!(&mut self.query, "<= ${}", param)
            }
            Operation::Gte => {
                let param = self.add_param(val);
                write!(&mut self.query, ">= ${}", param)
            }
            Operation::In => {
                let param = self.add_param(val);
                write!(&mut self.query, "= ANY (${})", param)
            }
            Operation::NotIn => {
                let param = self.add_param(val);
                write!(&mut self.query, "<> ANY (${})", param)
            }
            Operation::IsNull => {
                write!(&mut self.query, "IS NULL")
            }
            Operation::IsNotNull => {
                write!(&mut self.query, "IS NOT NULL")
            }
            Operation::Contains => {
                let param = self.add_param(val);
                write!(&mut self.query, "LIKE '%' || ${} || '%'", param)
            }
        }
        .unwrap();
    }

    fn on_join_column_filter(&mut self, f: &JoinColumnFilter<'v>) {
        let parent_table_id = self.current_table_id();
        let child_table_id = self.push_new_table();

        write!(
            &mut self.query,
            "EXISTS(SELECT TRUE FROM {} AS T{} WHERE T{}.{} = T{}.{}",
            f.filter().table_name(),
            child_table_id,
            child_table_id,
            f.child_column(),
            parent_table_id,
            f.parent_column()
        )
        .unwrap();

        if !f.filter().filter_group().is_empty() {
            self.query.push_str(" AND ");
            f.filter().accept(self);
        }

        self.query.push_str(")");
        self.pop_old_table();
    }

    fn on_join_table_filter(&mut self, f: &JoinTableFilter<'v>) {
        let parent_table_id = self.current_table_id();
        let join_table_id = self.push_new_table();
        let child_table_id = self.push_new_table();

        write!(
            &mut self.query,
            "EXISTS(SELECT TRUE FROM {} AS T{} INNER JOIN {} AS T{} ON T{}.{} = T{}.{} WHERE T{}.{} = T{}.{}",
            f.join_table(),
            join_table_id,
            f.filter().table_name(),
            child_table_id,
            child_table_id,
            f.child_table_join_col(),
            join_table_id,
            f.join_table_child_col(),
            join_table_id,
            f.join_table_parent_col(),
            parent_table_id,
            f.parent_table_join_col()
        ).unwrap();

        if !f.filter().filter_group().is_empty() {
            self.query.push_str(" AND ");
            f.filter().accept(self);
        }

        self.query.push_str(")");
        self.pop_old_table();
        self.pop_old_table();
    }
}
