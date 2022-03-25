use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(clap::Args)]
pub struct Opt {
    spec: Utf8PathBuf,

    #[clap(short, long)]
    config: Option<Utf8PathBuf>,

    #[clap(short, long)]
    workers: Option<usize>,

    #[clap(short, long)]
    coverage: Option<u32>,

    #[clap(short, long)]
    ignore_deadlock: bool,

    #[clap(short, long)]
    cleanup: bool,
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

    args.push(opt.spec.into());

    crate::exec_tla2tools(args)
}
