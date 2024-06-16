extern crate colored;

mod minix;
mod engine;
mod args_cli;

use colored::*;
use clap::Parser;
use figlet_rs::FIGfont;

use std::error::Error;

use crate::{
    args_cli::Flags,
    engine::engine::Engine,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let flags = Flags::parse();

    let author = env!("CARGO_PKG_AUTHORS");
    let homepage = env!("CARGO_PKG_HOMEPAGE");

    if let Some(title) = FIGfont::standard().unwrap().convert("Minix") {
        println!("{}", title.to_string().bold().blue());
        println!("-------------------------------------------------------------------");
        println!("Homepage: {} • {}", homepage.blue(), author.green());
        println!("-------------------------------------------------------------------");
    }

    Engine::run(
        &flags.input, flags.output.as_deref()
    )?;

    Ok(())
}
