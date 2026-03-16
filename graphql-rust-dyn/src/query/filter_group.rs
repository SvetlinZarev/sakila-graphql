use crate::query::filter_type::FilterType;
use crate::query::ops::Combinator;
use crate::query::visitor::Visitor;

#[derive(Debug, Clone)]
pub struct FilterGroup<'f> {
    combinator: Combinator,
    filters: Vec<FilterType<'f>>,
    groups: Vec<FilterGroup<'f>>,
}

impl<'f> FilterGroup<'f> {
    pub fn new(combinator: Combinator) -> Self {
        Self {
            combinator,
            filters: vec![],
            groups: vec![],
        }
    }

    pub fn accept(&self, v: &mut dyn Visitor<'f>) {
        v.on_filter_group(self);
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty() && self.groups.is_empty()
    }

    pub fn add_filter<T: Into<FilterType<'f>>>(&mut self, filter: T) {
        self.filters.push(filter.into());
    }

    pub fn add_group(&mut self, group: FilterGroup<'f>) {
        // nothing to do if the group is empty
        if group.is_empty() {
            return;
        }

        // groups of the same type can be compressed into one group
        if self.combinator == group.combinator {
            self.filters.extend(group.filters);
            for g in group.groups {
                self.add_group(g);
            }

            return;
        }

        // Special case: group with only one filter
        if group.groups.is_empty() && group.filters.len() == 1 {
            self.filters.extend(group.filters);
            return;
        }

        // compress a group with only one sub-group
        if group.filters.is_empty() && group.groups.len() == 1 {
            group.groups.into_iter().for_each(|g| self.add_group(g));
            return;
        }

        // cannot merge the new group, so add it as child
        self.groups.push(group);
    }

    pub fn filters(&self) -> &[FilterType<'f>] {
        &self.filters
    }

    pub fn groups(&self) -> &[FilterGroup<'f>] {
        &self.groups
    }

    pub fn combinator(&self) -> Combinator {
        self.combinator
    }
}
