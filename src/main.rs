use std::{
    error::Error,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use anyhow::anyhow;
use clap::Parser;
use filter::JsonFilter;
use json_value::JsonValue;

mod filter;
mod json_value;

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

    let parsed_input =
        json_value::parse_json(&input).map_err(|_| anyhow!("failed to parse input"))?;

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
            let parsed_value = json_value::parse_json(&value)?;
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

fn apply_filter<'a>(value: JsonValue<'a>, filter: JsonFilter) -> JsonValue<'a> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    /// make sure that the expected json is pretty printed
    fn test_json_filter(json_str: &str, filter_str: &str, expected_json_str: &str) {
        let json = json_value::parse_json(json_str).expect("Json should be valid");
        let filter = filter::parse_filter(filter_str).expect("Filter should be valid");
        let filtered_json = apply_filter(json, filter);

        assert_eq!(filtered_json.fmt_pretty().to_string(), expected_json_str);
    }

    const JSON_EXAMPLE_1: &'static str = include_str!("../test-json/example1.json");

    #[test]
    fn do_nothing_filter_test() {
        let filter = ".";
        let input = JSON_EXAMPLE_1;
        let expected = json_value::parse_json(JSON_EXAMPLE_1)
            .unwrap()
            .fmt_pretty()
            .to_string();

        test_json_filter(input, filter, &expected);
    }
}
