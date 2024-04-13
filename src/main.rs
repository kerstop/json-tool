use std::{
    error::Error,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use anyhow::anyhow;
use clap::Parser;

mod json_value;
mod parsing;

/// A tool for dealing with JSON
#[derive(clap::Parser, Debug)]
struct Args {
    /// format the output with no whitespace
    #[arg(short)]
    dense: bool,
    /// omit to format only
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// get a value
    Get { key: String },

    /// set a value
    Set {
        /// write to stdout instead of the file
        #[arg(short)]
        stdout: bool,
        key: String,
        value: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = std::io::read_to_string(stdin())?;

    let parsed_input = parsing::parse_json(&input).map_err(|_| anyhow!("failed to parse input"))?;

    let mut out = stdout().lock();

    match &args.command {
        None {} => match args.dense {
            true => write!(&mut out, "{}", parsed_input.fmt_dense())?,
            false => write!(&mut out, "{}", parsed_input.fmt_pretty())?,
        },
        Some(Command::Get { key }) => match args.dense {
            true => write!(&mut out, "{}", parsed_input.get_path(&key)?.fmt_dense())?,
            false => write!(&mut out, "{}", parsed_input.get_path(&key)?.fmt_pretty())?,
        },
        Some(Command::Set { key, value, .. }) => {
            let parsed_value = parsing::parse_json(&value)?;
            let mut parsed_input = parsed_input;

            *parsed_input.get_path_mut(&key)? = parsed_value;

            match args.dense {
                true => write!(&mut out, "{}", parsed_input.fmt_dense())?,
                false => write!(&mut out, "{}", parsed_input.fmt_pretty())?,
            }
        }
    }

    Ok(())
}
