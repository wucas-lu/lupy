use clap::*;
use lupy;
use std::{fs::File, io::Read, path::PathBuf};

/// General purpose Luau to Python transpiler.
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

  let mut file = File::open(args.files).expect("invalid file");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("cannot read file");

  println!("{:#?}", lupy::into_python(&contents));
}
