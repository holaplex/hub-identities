use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use crate::{graphql::object::user::User, AppContext};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Credential {
    #[graphql(external)]
    pub created_by_id: Uuid,
}

#[ComplexObject]
impl Credential {
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let AppContext { user_loader, .. } = ctx.data::<AppContext>()?;

        user_loader.load_one(self.created_by_id).await
    }
}
