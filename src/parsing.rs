use std::error::Error;

use anyhow::Context;
use pest::iterators::Pair;
use pest::Parser;

use crate::JsonValue;

#[derive(pest_derive::Parser)]
#[grammar = "src/json.pest"]
struct JsonParser;

/// parse a single json str
pub fn parse_json(json: &str) -> Result<JsonValue<'_>, Box<dyn Error>> {
    let mut file = JsonParser::parse(Rule::json_file, json)?.next().unwrap();
    debug_assert_eq!(file.as_rule(), Rule::json_file);

    return parse_value(file.into_inner().next().unwrap());
}

fn parse_value(p: Pair<'_, Rule>) -> Result<JsonValue<'_>, Box<dyn Error>> {
    debug_assert_eq!(p.as_rule(), Rule::value);
    let value = p.into_inner().next().unwrap();
    match value.as_rule() {
        Rule::object => parse_object(value),
        Rule::array => parse_array(value),
        Rule::string => parse_string(value),
        Rule::number => parse_number(value),
        Rule::boolean => parse_boolean(value),
        Rule::null => Ok(JsonValue::Null),
        _ => unreachable!(),
    }
}

fn parse_string(p: Pair<'_, Rule>) -> Result<JsonValue<'_>, Box<dyn Error>> {
    debug_assert_eq!(p.as_rule(), Rule::string);
    Ok(JsonValue::String(p.into_inner().next().unwrap().as_str()))
}

fn parse_boolean(p: Pair<'_, Rule>) -> Result<JsonValue<'_>, Box<dyn Error>> {
    debug_assert_eq!(p.as_rule(), Rule::boolean);
    match p.as_str() {
        "true" => Ok(JsonValue::Bool(true)),
        "false" => Ok(JsonValue::Bool(false)),
        _ => unreachable!(),
    }
}

fn parse_array(array: Pair<'_, Rule>) -> Result<JsonValue<'_>, Box<dyn Error>> {
    debug_assert_eq!(array.as_rule(), Rule::array);
    let mut values: Vec<JsonValue> = Vec::new();
    for value in array.into_inner() {
        values.push(parse_value(value)?);
    }
    Ok(JsonValue::Array(values))
}

fn parse_object(object: Pair<'_, Rule>) -> Result<JsonValue<'_>, Box<dyn Error>> {
    debug_assert_eq!(object.as_rule(), Rule::object);
    let mut object_pairs = Vec::new();
    for object_pair in object.into_inner() {
        let mut object_pair_values = object_pair.into_inner();
        let name = object_pair_values.next().context("parse object key")?.as_str();
        let value = parse_value(object_pair_values.next().context("parse object value")?)?;
        object_pairs.push((name, value));
    }

    Ok(JsonValue::Object(object_pairs))
}

fn parse_number(p: Pair<'_, Rule>) -> Result<JsonValue<'_>, Box<dyn Error>> {
    debug_assert_eq!(p.as_rule(), Rule::number);
    // TODO: This number parsing does not correctly reflect how
    // json stores numbers
    let n = p.as_str().parse::<f64>().context("parse number")?;
    Ok(JsonValue::Number(n))
}

#[cfg(test)]
mod test {
    use crate::JsonValue;

    use super::{parse_json, parse_value};

    const JSON_OBJECT: &'static str = r#"{
    "nesting": { "inner object": {} },
    "an array": [1.5, true, null, 1e-6],
    "string with escaped double quotes" : "\"quick brown foxes\""
}"#;

    const JSON_STRING: &'static str = "\"hello world\"";

    #[test]
    fn parser_test() {
        parse_json(JSON_OBJECT).expect("parse should succeed");
    }

    #[test]
    fn parse_string() {
        if let JsonValue::String(s) = parse_json(JSON_STRING).expect("parse should succeed") {
            assert_eq!(s, "hello world");
        } else {
            panic!("expected a string")
        };
    }

    #[test]
    fn parse_bools_and_null() {
        let bools_and_null = ["true", "false", "null"];
        let expected = [
            JsonValue::Bool(true),
            JsonValue::Bool(false),
            JsonValue::Null,
        ];

        for (&input, &ref expected) in bools_and_null.iter().zip(expected.iter()) {
            assert_eq!(parse_json(input).unwrap(), *expected);
        }
    }

    const JSON_ARRAY: &'static str = "[\"hello\", 123.0, false]";

    #[test]
    fn parse_array() {
        assert_eq!(
            parse_json(JSON_ARRAY).unwrap(),
            JsonValue::Array(vec![
                JsonValue::String("hello"),
                JsonValue::Number(123.0),
                JsonValue::Bool(false)
            ])
        );
    }
}
