use serde::Deserialize;

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
pub struct Tab {
    pub description: String,
    pub devtoolsFrontendUrl: String,
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub url: String,
    pub webSocketDebuggerUrl: String,
}
