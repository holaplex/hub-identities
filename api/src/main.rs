mod graphql;

use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use hub_identities_core::prelude::*;
use ory::kratos::client::Client;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::AddData,
    post,
    web::{Data, Html},
    EndpointExt, IntoResponse, Route, Server,
};

use crate::graphql::{
    dataloader,
    schema::{build_schema, AppState},
};

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[arg(short, long, env, default_value = "3001")]
    pub port: u16,

    #[command(flatten)]
    pub kratos: ory::kratos::client::KratosArgs,
}

struct AppContext {
    pub user_loader: DataLoader<dataloader::user::Loader>,
}

impl AppContext {
    pub fn new(kratos: Client) -> Self {
        Self {
            user_loader: DataLoader::new(dataloader::user::Loader::new(kratos), tokio::spawn),
        }
    }
}

#[handler]
async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[handler]
async fn health() {}

#[handler]
async fn graphql_handler(
    Data(state): Data<&AppState>,
    req: GraphQLRequest,
) -> poem::Result<GraphQLResponse> {
    let context = AppContext::new(state.kratos.clone());

    Ok(state
        .schema
        .execute(req.0.data(context).data(state.kratos.clone()))
        .await
        .into())
}

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv::dotenv().ok();
    }

    let Args { port, kratos } = Args::parse();

    env_logger::builder()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .parse_default_env()
        .init();

    let kratos = Client::new(kratos.kratos_admin_endpoint)?;

    let schema = build_schema();

    let state = AppState::new(schema, kratos);

    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .run(
            Route::new()
                .at("/health", get(health))
                .at("/graphql", post(graphql_handler).with(AddData::new(state)))
                .at("/playground", get(playground)),
        )
        .await
        .map_err(Into::into)
}
