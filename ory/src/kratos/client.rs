use anyhow::{Context, Result};
use reqwest::{Client as HttpClient, Url};
use serde::{de::DeserializeOwned, Deserialize};
use uuid::Uuid;

#[derive(Debug, clap::Args)]
pub struct KratosArgs {
    #[arg(long, env)]
    pub kratos_admin_endpoint: String,
}

/// Ory API client
#[derive(Clone)]
pub struct Client {
    base_endpoint: Url,
    http: HttpClient,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IdentityResponse<T> {
    pub id: uuid::Uuid,
    #[serde(rename(deserialize = "traits"))]
    pub identity_trait: T,
    pub created_at: String,
    pub updated_at: String,
    pub state: String,
}

impl Client {
    pub fn new(kratos_admin_endpoint: String) -> Result<Self> {
        let http = HttpClient::new();

        let base_endpoint =
            Url::parse(&kratos_admin_endpoint).context("failed to parse kratos admin endpoint")?;

        Ok(Self {
            base_endpoint,
            http,
        })
    }

    /// get a Kratos identities by its id
    pub async fn get_identity<T: DeserializeOwned>(&self, id: Uuid) -> Result<IdentityResponse<T>> {
        let path = format!("/admin/identities/{id}");
        let url = self.base_endpoint.join(&path)?;

        let req = self.http.get(url);

        let response = req.send().await?.text().await?;

        Ok(serde_json::from_str(&response)?)
    }

    /// list Kratos identities
    pub async fn list_identities<T: DeserializeOwned>(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<IdentityResponse<T>>> {
        let per_page = per_page.unwrap_or(250);
        let page = page.unwrap_or(1);

        let path = format!("/admin/identities?page={page}&per_page={per_page}");

        let url = self.base_endpoint.join(&path)?;

        let req = self.http.get(url);

        let response = req.send().await?.text().await?;

        Ok(serde_json::from_str(&response)?)
    }
}
