use crate::manifest::{Manifest, Tla2ToolsManifest};

use std::fs::{self, File};
use std::io::{self, Write};

use anyhow::{Context, Result};
use camino::Utf8Path;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use semver::Version;
use tempfile::NamedTempFile;

pub async fn run() -> Result<()> {
    let mut manifest = Manifest::load()?;

    {
        println!("updating tla2tools ...");

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

        let needs_download = match manifest.tla2tools {
            Some(ref m) => {
                println!("current version: {}", m.current_version);
                let old_path = manifest.tla2tools_current_path().unwrap();
                !old_path.exists() || m.current_version != latest_version
            }
            None => {
                println!("current version: none");
                true
            }
        };

        println!("latest  version: {latest_version}");

        if needs_download {
            let url = tla2tools_asset.browser_download_url.clone();
            let new_path = Manifest::tla2tools_jar_path(&latest_version);
            let msg = format!("downloading tla2tools v{latest_version}");
            download(url, &new_path, msg).await?;

            let old_version = manifest
                .tla2tools
                .as_ref()
                .map(|m| m.current_version.clone());
            let old_path = manifest.tla2tools_current_path();

            manifest.tla2tools = Some(Tla2ToolsManifest {
                current_version: latest_version.clone(),
            });
            manifest.save()?;

            if let Some(ref old_path) = old_path {
                if old_version.unwrap() != latest_version {
                    fs::remove_file(old_path).ok();
                }
            }

            #[cfg(target_os = "linux")]
            {
                let link_path = Manifest::tla2tools_dir().join("tla2tools.latest.jar");
                fs::remove_file(&link_path).ok();
                std::os::unix::fs::symlink(new_path, link_path)?;
            }

            println!("finished");
        }
    }

    {
        let jar_path = manifest.tla2tools_current_path().unwrap();
        let mut zip = zip::ZipArchive::new(File::open(jar_path)?)?;
        let mut src = zip.by_name("tla2tex/tlatex.sty")?;
        let mut dst = File::create(Manifest::tlatex_sty_path())?;
        io::copy(&mut src, &mut dst)?;
    }

    Ok(())
}

async fn download(url: Url, path: &Utf8Path, msg: String) -> Result<()> {
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
