mod logger;
mod rpc;
mod shell;

use clap::Parser;

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
    let client = LiveShareClient::connect(format!("http://{}", server_addr)).await?;

    shell::run(shell::ShellState { client }).await?;
    Ok(())
}
