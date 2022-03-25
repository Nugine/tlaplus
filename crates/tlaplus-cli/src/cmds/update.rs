use crate::manifest::Manifest;

use anyhow::Result;

pub fn run() -> Result<()> {
    let manifest = Manifest::load()?;
    dbg!(&manifest);
    Ok(())
}
