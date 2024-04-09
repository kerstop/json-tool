use std::fmt::{self, Formatter, Write};

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
}

struct PrettyFormatter<'a> {
    json: JsonValue<'a>,
}

impl fmt::Display for PrettyFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
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
                    self.format_pair(pairs.next().unwrap(), f)?;
                    for pair in pairs {
                        f.write_char(',')?;
                        self.format_pair(pair, f)?;
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
    fn format_pair(&self, (key, value): &(&str, JsonValue<'_>), f: &mut Formatter) -> fmt::Result {
        f.write_str(key)?;
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
