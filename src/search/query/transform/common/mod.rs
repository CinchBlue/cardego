use crate::search::query::ast::{AndExpressionGroup, Expression};
use anyhow::Result;
use std::collections::HashMap;
use std::error::Error;

pub mod sql;

impl Expression {
    pub fn from_query_string(query_string: &str) -> Result<Expression, Box<dyn Error>> {
        use crate::search::query::parser;

        let expr = parser::rules::expression(query_string)
            .map_err(|err| {
                anyhow!(
                    "Could not parse query string `{}` due to error `{:?}`",
                    query_string,
                    err
                )
            })?
            .1;
        Ok(expr)
    }

    pub fn split_query_by_name(
        &self,
        mappings: &HashMap<String, Vec<&str>>,
    ) -> HashMap<String, Expression> {
        // For each mapping
        // Preserve the original expression structure as much as possible.
        mappings
            .iter()
            .map(|(split_query_name, names_to_keep)| {
                // Take the original's list, clone it
                let filtered_and_expr_groups = self
                    .0
                    .clone()
                    .into_iter()
                    // For each and-expr-group
                    .map(|and_expr_group| {
                        let filtered_predicates = and_expr_group
                            .clone()
                            .0
                            .into_iter()
                            // For each predicate
                            // - Keep only the relevant mappings
                            .filter(|pred| names_to_keep.iter().any(|name| name == &pred.name))
                            .collect::<Vec<_>>();
                        AndExpressionGroup(filtered_predicates)
                    })
                    .collect::<Vec<_>>();
                let filtered_expr = Expression(filtered_and_expr_groups.to_vec());
                (split_query_name.clone(), filtered_expr)
            })
            .collect::<HashMap<String, Expression>>()
            .clone()
    }
}
