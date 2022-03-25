#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod config;
mod manifest;

mod translate;
mod update;

use anyhow::Result;
use clap::StructOpt;

#[derive(clap::Parser)]
#[non_exhaustive]
enum Opt {
    #[clap(alias = "u")]
    Update,
    #[clap(alias = "t")]
    Translate(translate::Opt),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    match opt {
        Opt::Update => update::run().await?,
        Opt::Translate(opt) => translate::run(opt).await?,
    }

    Ok(())
}
