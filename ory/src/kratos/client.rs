use reqwest;
use serde::Deserialize;
use clap::Parser;

/// Arguments for establishing a database connection
#[derive(Debug, Parser)]
pub struct KratosArgs {
    #[arg(long, env)]
    base_endpoint: String,
    #[arg(long, env)]
    authorization: String   
}

/// Ory API client
pub struct Client {
    base_endpoint: String,
    authorization: String,
}

#[derive(Deserialize)]
pub struct Identity {
    
}

impl Client {
    pub fn new(base_endpoint: String, authorization: String) -> Self {
        Self{ base_endpoint, authorization }
    }

    pub fn get_identity(&self) -> Result<Identity> {
        reqwest
    }
}
