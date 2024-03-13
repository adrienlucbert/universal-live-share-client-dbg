use std::error::Error;
use std::fmt;

use shellfish::{async_fn, Command};

use super::super::ShellState;
use crate::hello_world::HelloRequest;

const HELP: &str = "Say hello.";

pub fn as_command() -> shellfish::Command<ShellState> {
    Command::new_async(HELP.to_string(), async_fn!(ShellState, handler))
}

async fn handler(state: &mut ShellState, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let name = args.get(1).ok_or_else(|| Box::new(SayHelloError))?;
    let request = HelloRequest {
        name: name.to_string(),
    };
    let response = state.client.say_hello(request).await?;
    println!("{:?}", response);
    Ok(())
}

/// Greeting error
#[derive(Debug)]
pub struct SayHelloError;

impl fmt::Display for SayHelloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No name specified")
    }
}

impl Error for SayHelloError {}
