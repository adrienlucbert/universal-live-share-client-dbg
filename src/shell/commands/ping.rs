use std::error::Error;

use shellfish::{async_fn, Command};

use super::super::ShellState;
use crate::rpc::live_share::RawMessage;

const HELP: &str = "Ping.";

pub fn as_command() -> shellfish::Command<ShellState> {
    Command::new_async(HELP.to_string(), async_fn!(ShellState, handler))
}

async fn handler(state: &mut ShellState, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let request = RawMessage {
        message: "Ping".to_string(),
    };
    let response = state.client.ping(request).await?;
    println!("{:?}", response);
    Ok(())
}
