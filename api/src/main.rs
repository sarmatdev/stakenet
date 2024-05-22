use axum::routing::get;
use axum::Router;
use clap::Parser;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

mod routes;

use crate::routes::index;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Bind address for the server
    #[clap(long, env, default_value_t = SocketAddr::from_str("0.0.0.0:7001").unwrap())]
    bind_addr: SocketAddr,

    /// RPC url
    #[clap(long, env)]
    rpc_url: String,

    /// Program ID
    #[clap(long, env)]
    program_id: Pubkey,
}

struct AppState {
    pub rpc_client: RpcClient,
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();
    info!("args: {:#?}", args);

    info!("starting server at {}", args.bind_addr);

    let rpc_client = RpcClient::new(args.rpc_url.clone());

    let app_state = Arc::new(AppState { rpc_client });

    let router = Router::new().route("/", get(index)).with_state(app_state);

    info!("started rpc client at {}", args.rpc_url);

    axum::Server::bind(&args.bind_addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
