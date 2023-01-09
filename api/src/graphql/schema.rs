use async_graphql::{
    extensions::{ApolloTracing, Logger},
    EmptySubscription, Schema, EmptyMutation,
};
use hub_identities_core::prelude::*;
use ory::kratos::client::Client;

use crate::graphql::query::Query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Context {
    kratos: Client,
}

impl Context {
    pub fn new() -> Result<Self> {
        let kratos = Client::new()?;

        Ok(Self { kratos })
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(context: Context) -> Result<AppSchema> {
    // todo! Shared struct instead of db

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .data(context.kratos)
        .enable_federation()
        .finish();

    Ok(schema)
}
