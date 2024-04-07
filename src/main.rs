mod parsing;

/// A tool for dealing with JSON
#[derive(clap::Parser, Debug)]
struct Args {}

#[derive(Debug, PartialEq)]
enum JsonValue<'a> {
    String(&'a str),
    Number(f64),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
    Bool(bool),
    Null,
}

fn main() {
    println!("Hello, world!");
}
