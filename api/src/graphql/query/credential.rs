use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;
use crate::graphql::object::Credential;

#[derive(Default)]
pub struct Query;

#[Object(name = "CredentialQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_credential_by_created_by_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] created_by_id: Uuid,
    ) -> Result<Credential> {
        Ok(Credential { created_by_id })
    }
}
