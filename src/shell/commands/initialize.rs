use std::error::Error;

use shellfish::{async_fn, Command};

use super::super::ShellState;
use crate::rpc::live_share::InitializeParams;

const HELP: &str = "Initialize connection to server.";

pub fn as_command() -> shellfish::Command<ShellState> {
    Command::new_async(HELP.to_string(), async_fn!(ShellState, handler))
}

async fn handler(state: &mut ShellState, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let request = InitializeParams {
        client_info: None,
        capabilities: None,
    };
    let response = state.client.initialize(request).await?;
    println!("{:?}", response);
    Ok(())
}
