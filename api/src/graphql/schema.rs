use async_graphql::{
    extensions::{ApolloTracing, Logger},
    EmptyMutation, EmptySubscription, Schema,
};
use ory::kratos::client::Client;

use crate::graphql::query::Query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub kratos: Client,
}

impl AppState {
    pub fn new(schema: AppSchema, kratos: Client) -> Self {
        Self { schema, kratos }
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .finish()
}
