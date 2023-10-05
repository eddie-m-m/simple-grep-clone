use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
#[derive(Parser, Debug)]
struct Namer {
    /// The name of the person to greet
    name: String,
    /// The number of times to greet
    count: u8,
}

fn main() {
    let cli_args = Cli::parse();
    let name_args: Namer = Namer::parse();

    for _ in 0..name_args.count {
        println!("Hello {}!", name_args.name)
    }
}
