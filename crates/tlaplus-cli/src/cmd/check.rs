use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(clap::Args)]
pub struct Opt {
    spec: Utf8PathBuf,

    #[clap(long)]
    config: Option<Utf8PathBuf>,

    #[clap(long)]
    workers: Option<usize>,

    #[clap(long)]
    coverage: Option<u32>,

    #[clap(long)]
    ignore_deadlock: bool,

    #[clap(long)]
    cleanup: bool,

    #[clap(long)]
    meta_dir: Option<Utf8PathBuf>,

    #[clap(long)]
    user_file: Option<Utf8PathBuf>,
}

pub async fn run(opt: Opt) -> Result<()> {
    let mut args = vec!["tlc2.TLC".to_owned()];

    if let Some(config) = opt.config {
        args.push("-config".to_owned());
        args.push(config.into());
    }

    {
        args.push("-workers".to_owned());
        match opt.workers {
            Some(n) => args.push(n.to_string()),
            None => args.push("auto".to_owned()),
        }
    }

    if let Some(coverage) = opt.coverage {
        args.push("-coverage".to_owned());
        args.push(coverage.to_string());
    }

    if opt.ignore_deadlock {
        args.push("-deadlock".to_owned())
    }

    if opt.cleanup {
        args.push("-cleanup".to_owned())
    }

    if let Some(meta_dir) = opt.meta_dir {
        args.push("-metadir".to_owned());
        args.push(meta_dir.into());
    }

    if let Some(user_file) = opt.user_file {
        args.push("-userFile".to_owned());
        args.push(user_file.into());
    }

    args.push(opt.spec.into());

    crate::exec_tla2tools(args)
}
