use async_graphql::SimpleObject;
use ory::kratos::client::IdentityResponse;
use serde::Deserialize;
use hub_core::uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NameField {
    #[serde(rename(deserialize = "first"))]
    pub first_name: String,
    #[serde(rename(deserialize = "last"))]
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct IdentityTrait {
    pub name: Option<NameField>,
    pub email: String,
}

#[derive(SimpleObject, Clone)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: String,
    updated_at: String,
}

impl From<IdentityResponse<IdentityTrait>> for User {
    fn from(
        IdentityResponse {
            id,
            created_at,
            updated_at,
            identity_trait: IdentityTrait { email, name },
            ..
        }: IdentityResponse<IdentityTrait>,
    ) -> Self {
        let name = name.unwrap_or(NameField {
            first_name: String::new(),
            last_name: String::new(),
        });

        Self {
            id,
            first_name: name.first_name,
            last_name: name.last_name,
            email,
            created_at,
            updated_at,
        }
    }
}
