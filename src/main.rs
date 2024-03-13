mod logger;
mod rpc;
mod shell;

use clap::Parser;
use log::warn;

use rpc::live_share::live_share_client::LiveShareClient;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// gRPC server host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// gRPC server port
    #[arg(short = 'p', long)]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let args = Args::parse();

    let server_addr = format!("{}:{}", args.host, args.port);
    let mut client = LiveShareClient::connect(format!("http://{}", server_addr)).await?;

    match client.version(()).await?.get_ref().message.as_str() {
        rpc::live_share::PROTOCOL_VERSION => (),
        server_protocol_version => warn!(
            "Client protocol version ({}) differs from server's ({}).",
            rpc::live_share::PROTOCOL_VERSION,
            server_protocol_version
        ),
    }

    shell::run(shell::ShellState { client }).await?;
    Ok(())
}
