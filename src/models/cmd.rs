use serde_json::{Map, Value};

pub type MessageParameter = Value;

pub struct Cmd<'a> {
    pub method: &'a str,
    pub params: Map<String, MessageParameter>,
}
