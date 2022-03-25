use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(clap::Args)]
pub struct Opt {
    input: Utf8PathBuf,
}

pub async fn run(opt: Opt) -> Result<()> {
    let args = vec!["pcal.trans".to_owned(), opt.input.into()];
    crate::exec_tla2tools(args)
}
