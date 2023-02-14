use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use uuid::Uuid;

use crate::{graphql::object::user::User, AppContext};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Owner {
    #[graphql(external)]
    pub user_id: Uuid,
}

#[ComplexObject]
impl Owner {
    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let AppContext { user_loader, .. } = ctx.data::<AppContext>()?;

        user_loader.load_one(self.user_id).await
    }
}
