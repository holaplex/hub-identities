use async_graphql::{Context, Object, Result};
use ory::kratos::client::Client;

use crate::graphql::object::user::{IdentityTrait, User};

#[derive(Default)]
pub struct Query;

#[Object(name = "UserQuery")]
impl Query {
    #[graphql(entity)]
    async fn user(&self, ctx: &Context<'_>, #[graphql(key)] id: String) -> Result<User> {
        let kratos = ctx.data::<Client>()?;
        let identity_response = kratos.get_identity::<IdentityTrait>(id).await?;

        Ok(identity_response.into())
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 1)] page: i64,
        #[graphql(default = 250)] per_page: i64,
    ) -> Result<Vec<User>> {
        let kratos = ctx.data::<Client>()?;
        let identities_response = kratos
            .list_identities::<IdentityTrait>(Some(page), Some(per_page))
            .await?;

        Ok(identities_response.into_iter().map(Into::into).collect())
    }
}
