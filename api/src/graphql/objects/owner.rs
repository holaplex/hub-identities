use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use crate::{graphql::objects::user::User, AppContext};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Owner {
    #[graphql(external)]
    pub user_id: Uuid,
}

#[ComplexObject]
impl Owner {
    /// The user identity associated with the owner of the organization.
    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let AppContext { user_loader, .. } = ctx.data::<AppContext>()?;

        user_loader.load_one(self.user_id).await
    }
}
