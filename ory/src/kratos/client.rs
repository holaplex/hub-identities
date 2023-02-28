use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use ory_openapi_generated_client::{
    apis::{
        configuration::Configuration,
        identity_api::{get_identity, list_identities},
    },
    models::{Identity, IdentityState},
};
use serde::{de::DeserializeOwned, Deserialize};
use uuid::Uuid;

#[derive(Debug, clap::Args)]
pub struct KratosArgs {
    #[arg(long, env, default_value = "http://127.0.0.1:4434")]
    pub ory_base_url: String,
    #[arg(long, env)]
    pub ory_auth_token: String,
}

/// Ory API client
#[derive(Clone)]
pub struct Client {
    base_url: String,
    auth_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IdentityResponse<T> {
    pub id: uuid::Uuid,
    #[serde(rename(deserialize = "traits"))]
    pub identity_trait: T,
    pub created_at: String,
    pub updated_at: String,
    pub state: IdentityState,
}

impl Client {
    pub fn new(base_url: String, auth_token: String) -> Result<Self> {
        Ok(Self {
            base_url,
            auth_token,
        })
    }

    /// get a Kratos identities by its id
    pub async fn get_identity<T: DeserializeOwned>(&self, id: Uuid) -> Result<IdentityResponse<T>> {
        let configuration = Configuration {
            base_path: self.base_url.clone(),
            bearer_access_token: Some(self.auth_token.clone()),
            ..Default::default()
        };

        let get_identity_response = get_identity(&configuration, &id.to_string(), None).await?;

        get_identity_response.try_into()
    }

    /// list Kratos identities
    pub async fn list_identities<T: DeserializeOwned>(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<IdentityResponse<T>>> {
        let configuration = Configuration {
            base_path: self.base_url.clone(),
            bearer_access_token: Some(self.auth_token.clone()),
            ..Default::default()
        };

        let list_identities_response =
            list_identities(&configuration, per_page, page, None).await?;

        list_identities_response
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<IdentityResponse<T>>>>()
    }
}

impl<T: DeserializeOwned> TryFrom<Identity> for IdentityResponse<T> {
    type Error = Error;

    fn try_from(value: Identity) -> std::result::Result<Self, Self::Error> {
        let traits = value
            .traits
            .ok_or_else(|| anyhow!("no traits on the identity"))?;

        let identity_trait: T = serde_json::from_value(traits)?;

        let id = Uuid::from_str(&value.id).context("unable to convert identity id to uuid")?;

        let created_at = value
            .created_at
            .ok_or_else(|| anyhow!("no created at on the identity"))?;

        let updated_at = value
            .updated_at
            .ok_or_else(|| anyhow!("no updated at on the identity"))?;

        let state = value
            .state
            .ok_or_else(|| anyhow!("no state on the identity"))?;

        Ok(Self {
            id,
            identity_trait,
            created_at,
            updated_at,
            state,
        })
    }
}
