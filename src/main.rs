mod minix;
mod files;
mod args_cli;

use clap::Parser;

use std::error::Error;

use crate::{
    args_cli::Flags,
    files::engine::Engine,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let flags = Flags::parse();
    Engine::run(&flags.input, &flags.output)?;

    Ok(())
}
