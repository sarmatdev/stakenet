use axum::routing::get;
use axum::Router;
use clap::Parser;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

mod routes;
mod state;

use crate::routes::{root, validator_history};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Bind address for the server
    #[clap(long, env, default_value_t = SocketAddr::from_str("127.0.0.1:7001").unwrap())]
    bind_addr: SocketAddr,

    /// RPC url
    #[clap(long, env, default_value = "")]
    rpc_url: String,

    /// Program ID
    #[clap(
        long,
        env,
        default_value = "HistoryJTGbKQD2mRgLZ3XhqHnN811Qpez8X9kCcGHoa"
    )]
    program_id: Pubkey,
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();
    info!("args: {:#?}", args);

    info!("starting server at {}", args.bind_addr);

    let rpc_client = RpcClient::new(args.rpc_url.clone());

    let state = Arc::new(Mutex::new(state::StateInternal::new(rpc_client)));

    let router = Router::new()
        .route("/", get(root))
        .route("/validator_history/:vote_account", get(validator_history))
        .with_state(state);

    info!("started rpc client at {}", args.rpc_url);

    axum::Server::bind(&args.bind_addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
