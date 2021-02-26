// <identifier>         ::= ([A-z_]),(A-z0-9_)*
// <string>             ::= '“‘,<string-inner>*,'"'
// <string-inner>       ::= ...
// <name>               ::= <symbol>|<string>
// <integer_base10>     ::= [0-9]+
// <float>              ::= ([0-9]*),’.’,([0-9]+)
// <literal>            ::= <identifier>|<string>|<integer_base10>|<float>
// <operator>           ::= ’:’|’=’|’>’|’<’|’>=’|’<=’
// <predicate>          ::= <name>,<operator>,<literal>
// <and-conjunction>    ::= ','|' '
// <and-expression-group>     ::= <predicate>,((<ws>*),<or-conjunction>,(<ws>*),<predicate>)*
// <or-conjunction>    ::= '|'|'\n'
// <expression>         ::= <and-expression-group>,((<ws>*),<or-conjunction>,(<ws>*),<and-expression-group>)*

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
