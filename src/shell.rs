mod commands;

use rustyline::DefaultEditor;
use shellfish::{handler::DefaultAsyncHandler, Shell};
use tonic::transport::Channel;

use crate::hello_world::greeter_client::GreeterClient;

pub struct ShellState {
    pub client: GreeterClient<Channel>,
}

pub async fn run(state: ShellState) -> Result<(), Box<dyn std::error::Error>> {
    let mut shell = Shell::new_with_async_handler(
        state,
        "> ",
        DefaultAsyncHandler::default(),
        DefaultEditor::new()?,
    );

    shell.commands.insert("say-hello", commands::say_hello());
    Ok(shell.run_async().await?)
}
