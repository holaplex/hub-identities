use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;

use crate::graphql::objects::Member;

#[derive(Default)]
pub struct Query;

#[Object(name = "MemberQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_member_by_user_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] user_id: Uuid,
    ) -> Result<Member> {
        Ok(Member { user_id })
    }
}
