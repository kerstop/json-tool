use std::{error::Error, io::stdin, path::PathBuf};

use clap::Parser;

mod json_value;
mod parsing;

/// A tool for dealing with JSON
#[derive(clap::Parser, Debug)]
struct Args {
        #[arg(short)]
        dense: bool,
    /// file to operate on, blank for stdin
    file: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// format the input
    #[command(name = "fmt")]
    Format {
    },

    Get {
        key: String,
    },

    Set {
        key: String,
        value: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = match args.file {
        Some(path) => std::fs::read_to_string(path)?,
        None => std::io::read_to_string(stdin())?,
    };

    let parsed_input = parsing::parse_json(&input)?;

    match args.command {
        Command::Format {} => match args.dense {
            true => println!("{}", parsed_input.fmt_dense()),
            false => println!("{}", parsed_input.fmt_pretty()),
        },
        Command::Get { key } => match args.dense {
            true => println!("{}", parsed_input.get_path(&key)?.fmt_dense()),
            false => println!("{}", parsed_input.get_path(&key)?.fmt_pretty()),
        },
        Command::Set {..} => todo!(),
    }

    Ok(())
}
