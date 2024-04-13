use std::fmt::{self, Formatter, Write};

use anyhow::{anyhow, Result};
use pest::Parser;

const NEWLINE_CHAR: char = '\n';

#[derive(Debug, PartialEq)]
pub(crate) enum JsonValue<'a> {
    String(&'a str),
    Number(f64),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
    Bool(bool),
    Null,
}

impl<'a> JsonValue<'a> {
    pub fn fmt_dense(&'a self) -> impl fmt::Display + 'a {
        DenseFormatter { json: self }
    }

    pub fn fmt_pretty(&'a self) -> impl fmt::Display + 'a {
        PrettyFormatter {
            json: self,
            tabstring: String::from("  "),
        }
    }

    pub fn get_path(&self, path: &str) -> Result<&JsonValue<'a>> {
        use crate::parsing::{JsonParser, Rule};

        let pairs = JsonParser::parse(Rule::json_path, path)
            .map_err(|_| anyhow!("provided path is malformated \"{path}\""))?
            .next()
            .unwrap()
            .into_inner();

        let mut current_value = self;

        for pair in pairs {
            match pair.as_rule() {
                Rule::json_path_key => {
                    let key = pair.as_str();
                    current_value = current_value.get_object_key(key)?;
                }
                Rule::json_path_index => {
                    let index = pair.as_str().parse().unwrap();
                    current_value = current_value.get_array_index(index)?;
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(current_value)
    }

    pub fn get_object_key(&self, key: &str) -> Result<&JsonValue<'a>> {
        if let JsonValue::Object(o) = self {
            match o.iter().find(|(key_l, _)| *key_l == key) {
                Some((_, value)) => Ok(value),
                None => Err(anyhow!("key {} does not exist", key)),
            }
        } else {
            Err(anyhow!(
                "tried to index a value that was not an object with key {key}"
            ))
        }
    }

    pub fn get_array_index(&self, index: usize) -> Result<&JsonValue<'a>> {
        if let JsonValue::Array(a) = self {
            match a.get(index) {
                Some(v) => Ok(v),
                None => Err(anyhow!("Index {index} out of bounds")),
            }
        } else {
            Err(anyhow!(
                "tried to index a value that was not an object with key {index}"
            ))
        }
    }

    pub fn get_path_mut(&mut self, path: &str) -> Result<&mut JsonValue<'a>> {
        use crate::parsing::{JsonParser, Rule};

        let pairs = JsonParser::parse(Rule::json_path, path)
            .map_err(|_| anyhow!("provided path is malformated \"{path}\""))?
            .next()
            .unwrap()
            .into_inner();

        let mut current_value = self;

        for pair in pairs {
            match pair.as_rule() {
                Rule::json_path_key => {
                    let key = pair.as_str();
                    current_value = current_value.get_object_key_mut(key)?;
                }
                Rule::json_path_index => {
                    let index = pair.as_str().parse().unwrap();
                    current_value = current_value.get_array_index_mut(index)?;
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(current_value)
    }

    pub fn get_object_key_mut(&mut self, key: &str) -> Result<&mut JsonValue<'a>> {
        if let JsonValue::Object(o) = self {
            match o.iter_mut().find(|(key_l, _)| *key_l == key) {
                Some((_, value)) => Ok(value),
                None => Err(anyhow!("key {} does not exist", key)),
            }
        } else {
            Err(anyhow!(
                "tried to index a value that was not an object with key {key}"
            ))
        }
    }

    pub fn get_array_index_mut(&mut self, index: usize) -> Result<&mut JsonValue<'a>> {
        if let JsonValue::Array(a) = self {
            match a.get_mut(index) {
                Some(v) => Ok(v),
                None => Err(anyhow!("Index {index} out of bounds")),
            }
        } else {
            Err(anyhow!(
                "tried to index a value that was not an object with key {index}"
            ))
        }
    }
}

struct PrettyFormatter<'a> {
    json: &'a JsonValue<'a>,
    tabstring: String,
}

impl PrettyFormatter<'_> {
    fn fmt_value(&self, value: &JsonValue<'_>, numtabs: u32, f: &mut Formatter<'_>) -> fmt::Result {
        match value {
            JsonValue::String(s) => {
                f.write_char('"')?;
                f.write_str(s)?;
                f.write_char('"')?;
            }
            JsonValue::Number(n) => f.write_fmt(format_args!("{}", n))?,
            JsonValue::Array(a) => {
                f.write_char('[')?;
                if a.len() != 0 {
                    f.write_char(NEWLINE_CHAR)?;
                    let mut values = a.iter();
                    //vec should be contain at leat one element
                    self.write_indentation(numtabs + 1, f)?;
                    self.fmt_value(values.next().unwrap(), numtabs + 1, f)?;
                    for value in values {
                        f.write_char(',')?;
                        f.write_char(NEWLINE_CHAR)?;
                        self.write_indentation(numtabs + 1, f)?;
                        self.fmt_value(value, numtabs + 1, f)?;
                    }
                    f.write_char(NEWLINE_CHAR)?;
                };
                self.write_indentation(numtabs, f)?;
                f.write_char(']')?;
            }
            JsonValue::Object(o) => {
                f.write_char('{')?;
                if o.len() != 0 {
                    f.write_char(NEWLINE_CHAR)?;
                    let mut pairs = o.iter();
                    // should have at leat one pair because length is not 0
                    self.write_indentation(numtabs + 1, f)?;
                    self.fmt_pair(pairs.next().unwrap(), numtabs + 1, f)?;
                    for pair in pairs {
                        f.write_char(',')?;
                        f.write_char(NEWLINE_CHAR)?;
                        self.write_indentation(numtabs + 1, f)?;
                        self.fmt_pair(pair, numtabs + 1, f)?;
                    }
                    f.write_char(NEWLINE_CHAR)?;
                }
                self.write_indentation(numtabs, f)?;
                f.write_char('}')?;
            }
            JsonValue::Bool(b) => match b {
                true => f.write_str("true")?,
                false => f.write_str("false")?,
            },
            JsonValue::Null => f.write_str("null")?,
        };
        if numtabs == 0 {
            f.write_char('\n')?;
        }
        Ok(())
    }

    fn fmt_pair(
        &self,
        (key, value): &(&str, JsonValue<'_>),
        numtabs: u32,
        f: &mut Formatter,
    ) -> fmt::Result {
        f.write_char('"')?;
        f.write_str(key)?;
        f.write_char('"')?;
        f.write_char(':')?;
        f.write_char(' ')?;
        self.fmt_value(value, numtabs, f)?;
        Ok(())
    }

    fn write_indentation(&self, numtabs: u32, f: &mut Formatter) -> fmt::Result {
        for _ in 0..numtabs {
            f.write_str(&self.tabstring)?;
        }
        Ok(())
    }
}

impl fmt::Display for PrettyFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_value(self.json, 0, f)
    }
}

struct DenseFormatter<'a> {
    json: &'a JsonValue<'a>,
}

impl DenseFormatter<'_> {
    fn fmt_value(&self, value: &JsonValue<'_>, f: &mut Formatter<'_>) -> fmt::Result {
        match value {
            JsonValue::String(s) => f.write_str(s)?,
            JsonValue::Number(n) => f.write_fmt(format_args!("{}", n))?,
            JsonValue::Array(a) => {
                f.write_char('[')?;
                if a.len() != 0 {
                    let mut values = a.iter();
                    //vec should be contain at leat one element
                    self.fmt_value(values.next().unwrap(), f)?;
                    for value in values {
                        f.write_char(',')?;
                        self.fmt_value(value, f)?;
                    }
                };
                f.write_char(']')?;
            }
            JsonValue::Object(o) => {
                f.write_char('{')?;
                if o.len() != 0 {
                    let mut pairs = o.iter();
                    // should have at leat one pair because length is not 0
                    self.fmt_pair(pairs.next().unwrap(), f)?;
                    for pair in pairs {
                        f.write_char(',')?;
                        self.fmt_pair(pair, f)?;
                    }
                }
                f.write_char('}')?;
            }
            JsonValue::Bool(b) => match b {
                true => f.write_str("true")?,
                false => f.write_str("false")?,
            },
            JsonValue::Null => f.write_str("null")?,
        };
        Ok(())
    }

    fn fmt_pair(&self, (key, value): &(&str, JsonValue<'_>), f: &mut Formatter) -> fmt::Result {
        f.write_char('"')?;
        f.write_str(key)?;
        f.write_char('"')?;
        f.write_char(':')?;
        self.fmt_value(value, f)?;
        Ok(())
    }
}

impl fmt::Display for DenseFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_value(self.json, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_json;

    #[test]
    fn get_value_by_path() {
        let json_string = std::fs::read_to_string("test-json/test1.json").unwrap();
        let root_value = parse_json(&json_string).unwrap();

        assert_eq!(
            *root_value.get_path("hello").unwrap(),
            JsonValue::String("world")
        );
        assert_eq!(
            *root_value.get_path("foo[0].bar").unwrap(),
            JsonValue::String("baz")
        );
        assert_eq!(
            *root_value.get_path("foo[0].a").unwrap(),
            JsonValue::Number(1.0)
        );
        assert_eq!(
            *root_value.get_path("foo[0].b").unwrap(),
            JsonValue::Number(2.0)
        );
    }
}
