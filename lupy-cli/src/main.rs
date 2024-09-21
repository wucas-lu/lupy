use clap::*;
use std::path::PathBuf;

/**
  General purpose Luau to Python transpiler.
*/
#[derive(Parser, Debug)]
#[command(version, about = "General purpose Luau to Python transpiler.", long_about = None)]
struct Args {
  /// Luau files to be transpiled.
  files: PathBuf,

  /// Watch files for changes to be transpiled.
  #[arg(short, long, default_value_t = false)]
  watch: bool,
}

fn main() {
  let args = Args::parse();

  println!("{:?}", args);
}
