use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;

use crate::graphql::objects::Owner;

#[derive(Default)]
pub struct Query;

#[Object(name = "OwnerQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_owner_by_user_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] user_id: Uuid,
    ) -> Result<Owner> {
        Ok(Owner { user_id })
    }
}
