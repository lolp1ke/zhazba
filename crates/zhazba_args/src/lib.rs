use std::path::PathBuf;


#[derive(clap::Parser, Debug)]
#[clap(version)]
pub struct Args {
  pub workspace: Option<PathBuf>,
}
impl Args {
  pub fn new() -> Args {
    return clap::Parser::parse();
  }
}
