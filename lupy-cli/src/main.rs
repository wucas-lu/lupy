use clap::*;

/// General purpose Luau to Python transpiler.
#[derive(Parser, Debug)]
#[command(version, about = "General purpose Luau to Python transpiler.", long_about = None)]
struct Args {
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
