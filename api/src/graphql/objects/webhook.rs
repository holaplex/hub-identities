use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use crate::{graphql::objects::User, AppContext};

#[derive(SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct Webhook {
    #[graphql(external)]
    pub created_by_id: Uuid,
}

#[ComplexObject]
impl Webhook {
    /// The user who created the webhook.
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let AppContext { user_loader, .. } = ctx.data::<AppContext>()?;

        user_loader.load_one(self.created_by_id).await
    }
}
