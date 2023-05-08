use async_graphql::SimpleObject;
use hub_core::uuid::Uuid;
use ory::kratos::client::IdentityResponse;
use serde::Deserialize;

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
    pub profile_image: Option<String>,
}

/// A unique user identity across the entire Holaplex ecosystem. A user can be associated with multiple organizations, but they are not required to have separate login credentials.
#[derive(SimpleObject, Clone)]
pub struct User {
    /// The unique identifier for the user identity.
    id: Uuid,
    /// The first name of the user identity.
    first_name: String,
    /// The last name of the user identity.
    last_name: String,
    /// The email address associated with the user identity.
    email: String,
    /// The profile image associated with the user identity.
    profile_image: String,
    /// The timestamp in UTC when the user identity was created.
    created_at: String,
    /// The timestamp in UTC when the user identity was last updated.
    updated_at: String,
}

impl From<IdentityResponse<IdentityTrait>> for User {
    fn from(
        IdentityResponse {
            id,
            created_at,
            updated_at,
            identity_trait: IdentityTrait { email, name, profile_image },
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
            profile_image: profile_image.unwrap_or_default(),
            created_at,
            updated_at,
        }
    }
}
