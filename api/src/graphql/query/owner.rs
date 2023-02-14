use async_graphql::{Context, Object, Result};

use crate::graphql::object::owner::Owner;

#[derive(Default)]
pub struct Query;

#[Object(name = "OwnerQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_owner_by_user_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] user_id: uuid::Uuid,
    ) -> Result<Owner> {
        Ok(Owner { user_id })
    }
}
