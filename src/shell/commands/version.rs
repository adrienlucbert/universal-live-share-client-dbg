use shellfish::{async_fn, Command};

use super::super::ShellState;
use crate::rpc::traits::RpcClient;

use std::error::Error;

const HELP: &str = "Displays server protocol version.";

pub fn as_command() -> shellfish::Command<ShellState> {
    Command::new_async(HELP.to_string(), async_fn!(ShellState, handler))
}

async fn handler(state: &mut ShellState, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let response = state.client.version().await?;
    println!("{}", response);
    Ok(())
}
