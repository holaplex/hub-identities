use async_graphql::{EmptySubscription, Schema, extensions};
use hub_identities_core::prelude::*;

use crate::graphql::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> Result<AppSchema> {
    // todo! Shared struct instead of db

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(extensions::Logger)
        .finish();

    Ok(schema)
}