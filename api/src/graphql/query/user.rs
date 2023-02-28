use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;

use crate::{graphql::object::User, AppContext};

#[derive(Default)]
pub struct Query;

#[Object(name = "UserQuery")]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let AppContext { user_loader, .. } = ctx.data::<AppContext>()?;

        user_loader.load_one(id).await
    }

    #[graphql(entity)]
    async fn find_user_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Option<User>> {
        self.user(ctx, id).await
    }
}
