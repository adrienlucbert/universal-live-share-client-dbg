mod rpc;
mod shell;

use clap::Parser;
use jsonrpsee::ws_client::WsClientBuilder;
use log::warn;
use rpc::traits::RpcClient;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// JSON-RPC server host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// JSON-RPC server port
    #[arg(short = 'p', long)]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("none,universal_live_share_client_dbg=trace"),
    );

    let args = Args::parse();
    let server_addr = format!("{}:{}", args.host, args.port);
    let url = format!("ws://{}", server_addr);
    let client = WsClientBuilder::default().build(&url).await?;

    let server_protocol_version = client.version().await?;
    if server_protocol_version != rpc::traits::PROTOCOL_VERSION {
        warn!(
            "Client protocol version ({}) differs from server's ({}).",
            rpc::traits::PROTOCOL_VERSION,
            server_protocol_version
        );
    }

    shell::run(shell::ShellState { client }).await
}
