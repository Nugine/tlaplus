#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod config;
mod manifest;

mod cmd {
    pub mod check;
    pub mod translate;
    pub mod update;
}

use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use clap::StructOpt;

#[derive(clap::Parser)]
#[non_exhaustive]
enum Opt {
    #[clap(alias = "u")]
    Update,
    #[clap(alias = "t")]
    Translate(cmd::translate::Opt),
    #[clap(alias = "c")]
    Check(cmd::check::Opt),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    match opt {
        Opt::Update => cmd::update::run().await?,
        Opt::Translate(opt) => cmd::translate::run(opt).await?,
        Opt::Check(opt) => cmd::check::run(opt).await?,
    }

    Ok(())
}

pub(crate) fn exec_tla2tools(args: Vec<String>) -> Result<()> {
    use crate::config::Config;
    use crate::manifest::Manifest;

    let manifest = Manifest::load()?;
    let config = Config::load()?;

    let jar_path = manifest
        .tla2tools_current_path()
        .with_context(|| "Could not find tla2tools. Please update.")?;

    let mut cmd = Command::new("java");

    cmd.arg("-cp");
    cmd.arg(jar_path);

    if let Some(java_config) = config.java {
        cmd.args(java_config.args.into_iter());
    }

    cmd.args(args.into_iter());

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    println!("{:?}", cmd);

    let exit_status = cmd.spawn()?.wait()?;
    if !exit_status.success() {
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::prelude::ExitStatusExt;
            bail!(
                "pcal.trans failed. code = {:?}, signal = {:?}",
                exit_status.code(),
                exit_status.signal()
            );
        }
        #[cfg(not(target_os = "linux"))]
        {
            todo!()
        }
    }

    Ok(())
}
