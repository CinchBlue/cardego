use crate::search::query::ast::{AndExpressionGroup, Expression, Literal, Operator, Predicate};

impl Expression {
    pub fn to_sql_where_string(&self) -> String {
        if self.0.len() <= 0 {
            return "".to_string();
        }

        if self.0.len() == 1 && self.0[0].0.len() == 0 {
            return "".to_string();
        }

        let mut result = "WHERE ".to_owned();

        let where_clauses: String = self
            .0
            .iter()
            .map(|and_expression_group: &AndExpressionGroup| {
                and_expression_group
                    .0
                    .iter()
                    .map(Predicate::to_sql_string)
                    .collect::<Vec<String>>()
                    .join(" AND ")
            })
            .map(|s| format!("({})", s))
            .collect::<Vec<String>>()
            .join(" OR ");
        result.push_str(&where_clauses);
        result
    }
}

impl Predicate {
    pub fn to_sql_string(&self) -> String {
        let mut transformed_literal = self.literal.clone();
        if Operator::LikeMatch == self.op || Operator::NotLikeMatch == self.op {
            if let Literal::String(s) = transformed_literal {
                transformed_literal = Literal::String(s.replace("*", "%"));
            }
        }
        format!(
            "`{}`{}{}",
            self.name,
            self.op.to_sql_string(),
            transformed_literal.to_sql_string()
        )
    }
}

impl Operator {
    pub fn to_sql_string(&self) -> &str {
        match self {
            Operator::LikeMatch => " LIKE ",
            Operator::NotLikeMatch => " NOT LIKE ",
            Operator::Equal => "=",
            Operator::NotEqual => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterOrEqual => ">=",
            Operator::LessOrEqual => "<=",
        }
    }
}

impl Literal {
    // TODO: This is fundamentally broken if it ever hits characters that need to be escaped.
    pub fn to_sql_string(&self) -> String {
        match self {
            Literal::String(s) => format!("\'{}\'", s),
            Literal::Integer(i) => format!("{}", i),
            Literal::Float(f) => format!("{}", f),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::search::query::parser::rules::expression;

    #[test]
    fn test_expression() {
        let input = "a=1, b=2, c!=3; input>=5; customer_name=aditya";
        assert_eq!(
            expression(input).unwrap().1.to_sql_where_string(),
            "WHERE (`a`=1 AND `b`=2 AND `c`!=3) OR (`input`>=5) OR (`customer_name`=\'aditya\')"
                .to_owned()
        );
    }
}
