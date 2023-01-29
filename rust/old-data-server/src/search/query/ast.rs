#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    LikeMatch,
    NotLikeMatch,
    Equal,
    NotEqual,
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

#[derive(Debug, Clone, PartialEq)]
pub struct AndExpressionGroup(pub Vec<Predicate>);

#[derive(Debug, Clone, PartialEq)]
pub struct Expression(pub Vec<AndExpressionGroup>);
