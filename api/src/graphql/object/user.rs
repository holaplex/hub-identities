use async_graphql::SimpleObject;
use hub_identities_core::prelude::*;
use ory::kratos::client::IdentityResponse;

#[derive(Debug, Deserialize)]
pub struct NameField {
    #[serde(rename(deserialize = "first"))]
    pub first_name: String,
    #[serde(rename(deserialize = "last"))]
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct IdentityTrait {
    pub name: NameField,
    pub email: String,
}

#[derive(SimpleObject)]
pub struct User {
    id: uuid::Uuid,
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
            identity_trait:
                IdentityTrait {
                    email,
                    name:
                        NameField {
                            first_name,
                            last_name,
                        },
                },
            ..
        }: IdentityResponse<IdentityTrait>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            created_at,
            updated_at,
        }
    }
}
