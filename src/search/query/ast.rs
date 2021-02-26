
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    LikeMatch,
    Equal,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Predicate {
    pub name: String,
    pub op: Operator,
    pub literal: Literal,
}

pub type AndExpressionGroup = Vec<Predicate>;
pub type Expression = Vec<AndExpressionGroup>;
