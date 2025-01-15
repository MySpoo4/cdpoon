#[macro_export]
macro_rules! params {
    ($( $key:literal => $value:expr ),*) => {{
        let mut map = serde_json::Map::new();
        $(
            map.insert($key.to_string(), $crate::models::MessageParameter::from($value));
        )*
        map
    }};
}
