mod graphql;

use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use hub_core::{clap, tokio};
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

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
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

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-credentials",
    };

    hub_core::run(opts, |common, args| {
        let Args { port, kratos } = args;

        common.rt.block_on(async move {
            let kratos = Client::new(kratos.ory_base_url, kratos.ory_auth_token)?;

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
        })
    })
}
