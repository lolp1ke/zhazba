use std::path::PathBuf;

use clap::Parser;


#[derive(clap::Parser, Debug)]
#[clap(version)]
pub struct Args {
  pub workspace: Option<PathBuf>,
}
impl Args {
  pub fn new() -> Args {
    return Args::parse();
  }
}
