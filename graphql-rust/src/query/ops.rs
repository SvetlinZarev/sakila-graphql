use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Combinator {
    And,
    Or,
}

impl Display for Combinator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Combinator::And => "AND",
            Combinator::Or => "OR",
        };

        f.write_str(s)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Contains,
}
