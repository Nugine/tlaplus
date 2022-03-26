use crate::manifest::Manifest;

use anyhow::Result;

pub async fn run() -> Result<()> {
    let home = Manifest::home_dir();
    println!("{home}");
    Ok(())
}
