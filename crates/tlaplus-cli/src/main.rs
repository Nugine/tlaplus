#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    if env::var_os("RUST_BACKTRACE").is_none() {
        env::set_var("RUST_BACKTRACE", "1")
    }

    let home = match tlaplus_home_dir() {
        Some(h) => h,
        None => bail!("Could not find tlaplus home directory"),
    };

    fs::create_dir_all(home)?;

    Ok(())
}

fn home_dir() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        env::var_os("HOME").map(PathBuf::from)
    }
    #[cfg(not(target_os = "linux"))]
    {
        todo!()
    }
}

fn tlaplus_home_dir() -> Option<PathBuf> {
    let home = env::var_os("TLAPLUS_HOME").map(PathBuf::from);
    if home.is_some() {
        return home;
    }

    let mut home = home_dir();
    if let Some(ref mut p) = home {
        p.push(".tlaplus");
        return home;
    }

    None
}
