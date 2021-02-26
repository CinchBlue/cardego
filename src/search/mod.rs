extern crate juniper;

pub mod parser;

use juniper::FieldResult;

use self::juniper::{EmptyMutation, RootNode};
use crate::database::DatabaseContext;
use crate::models::{Card, FullCardData};

pub struct GraphQLContext;

// To make our context useable by Juniper, we have to implement a marker trait.
impl juniper::Context for DatabaseContext {}

// Root GraphQL query
pub struct QueryRoot;

// The root Query struct relies on GraphQLContext
#[juniper::object(
// Here we specify the context type of the object.
// We need to do this ine ggvery type that has access to the context.
Context = DatabaseContext,
)]
impl QueryRoot {
    fn api_version() -> &str {
        "0.9.0"
    }

    fn card(context: &DatabaseContext, id: i32) -> FieldResult<Card> {
        Ok(context.get_card(id)?)
    }

    fn full_card_data(context: &DatabaseContext, id: i32) -> FieldResult<FullCardData> {
        Ok(context.get_full_card_data(id)?)
    }
}

pub struct MutationRoot;

// TODO: Figure out how to use mutations properly. Just don't use mutations
// right now.
#[juniper::object(
Context = DatabaseContext,
)]
impl MutationRoot {
    //fn create_full_card_data(
    //    context: &DatabaseContext,
    //    full_card_data: NewFullCardData)
    //    -> FieldResult<FullCardData> {
    //    Ok(context.create_card(&full_card_data)?)
    //}
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<DatabaseContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new())
}
