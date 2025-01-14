extern crate colored;

mod core;
mod minix;
mod args_cli;

use colored::*;
use clap::Parser;
use figlet_rs::FIGfont;

use std::error::Error;

use crate::{
    minix::Minix,
    args_cli::Flags,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");
    let homepage = env!("CARGO_PKG_HOMEPAGE");

    if let Some(title) = FIGfont::standard().unwrap().convert(name) {
        println!("{}", title.to_string().bold().yellow());
        println!("v.{} | {} | @{}", version.bold().blue(), homepage.bold().cyan(), author.bold().green());
        println!("");
    }

    Minix::new(
        Flags::parse()
    ).run()?;

    Ok(())
}
