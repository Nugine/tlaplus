use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(clap::Args)]
pub struct Opt {
    input: Utf8PathBuf,

    #[clap(short, long)]
    shade: bool,

    #[clap(short, long)]
    number: bool,

    #[clap(short, long)]
    pt_size: Option<u8>,
}

pub async fn run(opt: Opt) -> Result<()> {
    let mut args = vec!["tla2tex.TLA".to_owned()];

    if opt.shade {
        args.push("-shade".to_owned());
    }

    if opt.number {
        args.push("-number".to_owned());
    }

    if let Some(pt_size) = opt.pt_size {
        args.push("-ptSize".to_owned());
        args.push(pt_size.to_string());
    }

    args.push(opt.input.into());

    crate::exec_tla2tools(args)
}
