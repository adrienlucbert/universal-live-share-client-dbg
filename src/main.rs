use clap::Parser;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::ws_client::WsClientBuilder;

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

#[rpc(client)]
pub trait Rpc {
    /// Ping.
    #[method(name = "ping")]
    async fn ping(&self) -> Result<String, ErrorObjectOwned>;
}

struct ShellState {
    client: jsonrpsee::ws_client::WsClient,
}

use std::error::Error;

use rustyline::DefaultEditor;
use shellfish::{async_fn, handler::DefaultAsyncHandler, Command, Shell};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let server_addr = format!("{}:{}", args.host, args.port);
    let url = format!("ws://{}", server_addr);
    let client = WsClientBuilder::default().build(&url).await?;

    let mut shell = Shell::new_with_async_handler(
        ShellState { client },
        "> ",
        DefaultAsyncHandler::default(),
        DefaultEditor::new()?,
    );

    shell.commands.insert(
        "ping",
        Command::new_async(
            "Displays a plaintext file.".to_string(),
            async_fn!(ShellState, ping),
        ),
    );

    shell.run_async().await?;
    Ok(())
}

async fn ping(state: &mut ShellState, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let connection_id_first = state.client.ping().await?;
    println!("{}", connection_id_first);
    Ok(())
}
