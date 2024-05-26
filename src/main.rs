mod minix;
mod engine;
mod args_cli;

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

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Minix");
    println!("{}", figure.unwrap());

    Engine::run(&flags.input, flags.output.as_deref())?;

    Ok(())
}
