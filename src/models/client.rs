use serde_json::{Map, Value};

pub type MessageParameter = Value;

pub type ClientResponse = Map<String, Value>;

pub struct Cmd<'a> {
    pub method: &'a str,
    pub params: Map<String, MessageParameter>,
}

pub struct Event<'a> {
    pub method: &'a str,
    pub params: Map<String, MessageParameter>,
}

impl<'a> Event<'a> {
    pub fn matches(&self, msg: &Value) -> bool {
        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
            if method == self.method {
                if let Some(params) = msg.get("params").and_then(|p| p.as_object()) {
                    return self
                        .params
                        .iter()
                        .all(|(key, value)| params.get(key).map_or(false, |v| v == value));
                }
            }
        }
        false
    }
}
