use anyhow::Result;

#[derive(clap::Args)]
pub struct Opt {
    #[clap(short, long)]
    help: bool,
}

pub async fn run(opt: Opt) -> Result<()> {
    let mut args = vec!["tlc2.TLC"];

    if opt.help {
        args.push("-help");
    }

    crate::exec_tla2tools(args)
}
