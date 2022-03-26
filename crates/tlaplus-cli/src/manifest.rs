use std::env::VarError;
use std::{env, fs};

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
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

    pub fn home_dir() -> &'static Utf8Path {
        static HOME: OnceCell<Utf8PathBuf> = OnceCell::new();
        HOME.get_or_init(|| {
            let home = match tlaplus_home_dir() {
                Some(h) => h,
                None => panic!("Could not find tlaplus home directory"),
            };

            fs::create_dir_all(&home).ok();
            assert!(home.exists());
            home
        })
    }

    pub fn tla2tools_dir() -> &'static Utf8Path {
        static PATH: OnceCell<Utf8PathBuf> = OnceCell::new();
        let home = Self::home_dir();
        PATH.get_or_init(|| {
            let tla2tools_dir = home.join("tla2tools");
            fs::create_dir_all(&tla2tools_dir).ok();
            assert!(tla2tools_dir.exists());
            tla2tools_dir
        })
    }

    pub fn tla2tools_jar_path(version: &Version) -> Utf8PathBuf {
        Self::tla2tools_dir().join(format!("tla2tools.v{version}.jar"))
    }

    pub fn path() -> &'static Utf8Path {
        static PATH: OnceCell<Utf8PathBuf> = OnceCell::new();
        let home = Self::home_dir();
        PATH.get_or_init(|| home.join("manifest.json"))
    }

    pub fn load() -> Result<Self> {
        let manifest_path = Self::path();
        let exists = manifest_path.exists();

        let manifest = if exists {
            serde_json::from_str(&fs::read_to_string(&manifest_path)?)?
        } else {
            Self::empty()
        };

        if !exists {
            manifest.save()?;
        }

        Ok(manifest)
    }

    pub fn save(&self) -> Result<()> {
        let file = fs::File::create(Self::path())?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn tla2tools_current_path(&self) -> Option<Utf8PathBuf> {
        let m = self.tla2tools.as_ref()?;
        Some(Self::tla2tools_jar_path(&m.current_version))
    }
}

fn env_var_path(key: &str) -> Option<Utf8PathBuf> {
    match env::var(key) {
        Ok(s) => Some(Utf8PathBuf::from(s)),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("expected utf8 environment variable, found {key}={s:?}")
        }
    }
}

fn home_dir() -> Option<Utf8PathBuf> {
    #[cfg(target_os = "linux")]
    {
        env_var_path("HOME")
    }
    #[cfg(not(target_os = "linux"))]
    {
        todo!()
    }
}

fn tlaplus_home_dir() -> Option<Utf8PathBuf> {
    let home = env_var_path("TLAPLUS_HOME");
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
