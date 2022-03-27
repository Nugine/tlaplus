use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(clap::Args)]
pub struct Opt {
    input: Utf8PathBuf,
}

pub async fn run(opt: Opt) -> Result<()> {
    let args = vec!["tla2sany.SANY".to_owned(), opt.input.into()];
    crate::exec_tla2tools(args)
}
