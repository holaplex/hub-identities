use async_graphql::{Context, Object, Result};

use crate::graphql::object::member::Member;

#[derive(Default)]
pub struct Query;

#[Object(name = "MemberQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_member_by_user_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] user_id: uuid::Uuid,
    ) -> Result<Member> {
        Ok(Member { user_id })
    }
}
