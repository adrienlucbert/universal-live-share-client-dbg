mod commands;

use rustyline::DefaultEditor;
use shellfish::{handler::DefaultAsyncHandler, Shell};
use tonic::transport::Channel;

use crate::rpc::live_share::live_share_client::LiveShareClient;

pub struct ShellState {
    pub client: LiveShareClient<Channel>,
}

pub async fn run(state: ShellState) -> Result<(), Box<dyn std::error::Error>> {
    let mut shell = Shell::new_with_async_handler(
        state,
        "> ",
        DefaultAsyncHandler::default(),
        DefaultEditor::new()?,
    );

    shell.commands.insert("ping", commands::ping());
    shell.commands.insert("version", commands::version());
    Ok(shell.run_async().await?)
}
