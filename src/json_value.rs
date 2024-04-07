#[derive(Debug, PartialEq)]
pub(crate) enum JsonValue<'a> {
    String(&'a str),
    Number(f64),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
    Bool(bool),
    Null,
}