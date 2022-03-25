#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod manifest;
mod cmds {
    pub mod translate;
    pub mod update;
}

use std::env;

use anyhow::Result;
use clap::StructOpt;

#[derive(clap::Parser)]
#[non_exhaustive]
enum Command {
    Update,
    Translate,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    if env::var_os("RUST_BACKTRACE").is_none() {
        env::set_var("RUST_BACKTRACE", "1")
    }

    let cmd = Command::parse();

    match cmd {
        Command::Update => cmds::update::run().await?,
        Command::Translate => todo!(),
    }

    Ok(())
}
