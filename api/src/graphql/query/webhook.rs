use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;

use crate::graphql::objects::Webhook;

#[derive(Default)]
pub struct Query;

#[Object(name = "WebhookQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_webhook_by_created_by_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] created_by_id: Uuid,
    ) -> Result<Webhook> {
        Ok(Webhook { created_by_id })
    }
}
