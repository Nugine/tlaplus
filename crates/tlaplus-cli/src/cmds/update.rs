use crate::manifest::{Manifest, Tla2ToolsManifest};

use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use semver::Version;
use tempfile::NamedTempFile;

pub async fn run() -> Result<()> {
    let mut manifest = Manifest::load()?;

    let gh = octocrab::instance();

    let release = gh
        .repos("tlaplus", "tlaplus")
        .releases()
        .get_latest()
        .await?;

    let latest_version = Version::parse(&release.tag_name.replace('v', "")).unwrap();

    let tla2tools_asset = release
        .assets
        .iter()
        .find(|a| a.name == "tla2tools.jar")
        .with_context(|| "Could not find tla2tools.jar in the latest release")?;

    let tla2tools_dir = Manifest::home_dir().join("tla2tools");
    fs::create_dir_all(&tla2tools_dir)?;

    let needs_download = match manifest.tla2tools {
        Some(m) => {
            println!("current version: {}", m.current_version);
            m.current_version != latest_version
        }
        None => {
            println!("current version: none");
            true
        }
    };

    println!("latest  version: {latest_version}");

    if needs_download {
        let url = tla2tools_asset.browser_download_url.clone();
        let path = tla2tools_dir.join(format!("tla2tools.{latest_version}.jar"));
        let msg = format!("downloading tla2tools v{latest_version}");
        download(url, &path, msg).await?;

        manifest.tla2tools = Some(Tla2ToolsManifest {
            current_version: latest_version,
        });
        manifest.save()?;
    }

    println!("finished");

    Ok(())
}

async fn download(url: Url, path: &Path, msg: String) -> Result<()> {
    let mut file = NamedTempFile::new()?;

    let resp = reqwest::get(url).await?;

    let pb = progress_bar(resp.content_length().unwrap(), msg);

    let mut stream = resp.bytes_stream();
    while let Some(result) = stream.next().await {
        let bytes = result?;
        let len: u64 = bytes.len().try_into().unwrap();
        file.write_all(&*bytes)?;
        pb.inc(len);
    }

    let temp_path = file.into_temp_path();
    fs::rename(temp_path, path)?;

    Ok(())
}

fn progress_bar(len: u64, msg: String) -> ProgressBar {
    let template ="{msg}\n[eta: {eta_precise}] [speed: {binary_bytes_per_sec}] [{percent}%] [{bytes}/{total_bytes}] {bar:40.cyan/blue}";
    let style = ProgressStyle::default_bar()
        .template(template)
        .progress_chars("##-");
    let pb = ProgressBar::new(len);
    pb.set_style(style);
    pb.set_message(msg);
    pb
}
