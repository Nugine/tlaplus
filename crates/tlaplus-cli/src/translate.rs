use crate::config::Config;
use crate::manifest::Manifest;

use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

#[derive(clap::Args)]
pub struct Opt {
    input: PathBuf,
}

pub async fn run(opt: Opt) -> Result<()> {
    let manifest = Manifest::load()?;
    let config = Config::load()?;

    let jar_path = manifest
        .tla2tools_current_path()
        .with_context(|| "Could not find tla2tools. Please update.")?;

    let mut cmd = Command::new("java");

    cmd.arg("-cp").arg(jar_path);

    if let Some(ref java_config) = config.java {
        cmd.args(java_config.args.iter());
    }

    cmd.arg("pcal.trans");

    cmd.arg(opt.input);

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

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
