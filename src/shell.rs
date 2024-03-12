mod commands;

use rustyline::DefaultEditor;
use shellfish::{handler::DefaultAsyncHandler, Shell};

pub struct ShellState {
    pub client: jsonrpsee::ws_client::WsClient,
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
