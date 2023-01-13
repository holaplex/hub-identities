use async_graphql::{Context, Object, Result};
use ory::kratos::client::Client;

use crate::graphql::object::user::{IdentityTrait, User};

#[derive(Default)]
pub struct Query;

#[Object(name = "UserQuery")]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: uuid::Uuid) -> Result<User> {
        let kratos = ctx.data::<Client>()?;
        let identity_response = kratos.get_identity::<IdentityTrait>(id).await?;

        Ok(identity_response.into())
    }

    #[graphql(entity)]
    async fn find_user_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: uuid::Uuid,
    ) -> Result<User> {
        self.user(ctx, id).await
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
