use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Pull,
    Play,
}

fn main() {
    let args = Args::parse();
}
