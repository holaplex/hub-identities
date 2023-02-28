use std::collections::HashMap;

use async_graphql::{
    dataloader::Loader as DataLoader, futures_util::future::try_join_all, FieldError, Result,
};
use hub_core::uuid::Uuid;
use ory::kratos::client::Client;
use poem::async_trait;

use crate::graphql::object::User;

#[derive(Clone)]
pub struct Loader {
    pub kratos: Client,
}

impl Loader {
    #[must_use]
    pub fn new(kratos: Client) -> Self {
        Self { kratos }
    }
}

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = User;

    async fn load(&self, user_ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let requests = user_ids
            .iter()
            .map(|user_id| self.kratos.get_identity(*user_id));
        let users = try_join_all(requests).await?;

        Ok(users
            .into_iter()
            .map(|identity| (identity.id, identity.into()))
            .collect())
    }
}
