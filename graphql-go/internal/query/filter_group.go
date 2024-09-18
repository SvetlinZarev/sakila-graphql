package query

type FilterGroup struct {
	Combinator Combinator
	Filters    []Filter
	Groups     []FilterGroup
}

func (f *FilterGroup) Accept(v Visitor) {
	v.onFilterGroup(f)
}

func (f *FilterGroup) IsEmpty() bool {
	return len(f.Filters) == 0 && len(f.Groups) == 0
}

func (f *FilterGroup) AddFilter(v Filter) {
	f.Filters = append(f.Filters, v)
}

func (f *FilterGroup) AddGroup(group FilterGroup) {
	// nothing to do if the group is empty
	if group.IsEmpty() {
		return
	}

	// groups of the same type can be compressed into one group
	if f.Combinator == group.Combinator {
		f.Filters = append(f.Filters, group.Filters...)
		for _, g := range group.Groups {
			f.AddGroup(g)
		}

		return
	}

	// Special case: group with only one filter
	if len(group.Groups) == 0 && len(group.Filters) == 1 {
		f.Filters = append(f.Filters, group.Filters...)
		return
	}

	// compress a group with only one sub-group
	if len(group.Groups) == 1 && len(group.Filters) == 0 {
		f.AddGroup(group.Groups[0])
		return
	}

	// cannot merge the new group, so add it as child
	f.Groups = append(f.Groups, group)
}
