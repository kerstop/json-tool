use std::path::PathBuf;

use clap::Parser;

mod json_value;
mod parsing;

/// A tool for dealing with JSON
#[derive(clap::Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Format {
        file: Option<PathBuf>,
    },

    Get {
        file: Option<PathBuf>,
        key: String,
    },

    Set {
        file: Option<PathBuf>,
        key: String,
        value: String,
    },
}

fn main() {
    let args = Args::parse();
}
