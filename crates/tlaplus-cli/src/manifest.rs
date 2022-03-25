use std::path::PathBuf;
use std::{env, fs, io};

use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub tla2tools: Option<Tla2ToolsManifest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tla2ToolsManifest {
    pub current_version: Version,
}

impl Manifest {
    pub fn empty() -> Self {
        Self { tla2tools: None }
    }

    pub fn load() -> Result<Self> {
        static HOME: OnceCell<PathBuf> = OnceCell::new();

        let home = HOME.get_or_try_init(|| {
            let home = match tlaplus_home_dir() {
                Some(h) => h,
                None => bail!("Could not find tlaplus home directory"),
            };

            fs::create_dir_all(&home)?;
            Ok(home)
        })?;

        let manifest_path = home.join("manifest.json");
        let exists = manifest_path.exists();

        let manifest = if exists {
            serde_json::from_str(&fs::read_to_string(&manifest_path)?)?
        } else {
            Self::empty()
        };

        if !exists {
            let file = fs::File::create(&manifest_path)?;
            write_pretty_json(file, &Self::empty())?;
        }

        Ok(manifest)
    }
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

fn write_pretty_json<W, T>(writer: W, value: &T) -> io::Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let formatter = serde_json::ser::PrettyFormatter::with_indent("   ".as_ref());
    let mut ser = serde_json::ser::Serializer::with_formatter(writer, formatter);
    value.serialize(&mut ser)?;
    Ok(())
}
