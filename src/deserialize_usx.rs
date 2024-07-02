use serde::Deserialize;

fn default_type() -> String {
    "usx".to_string()
}

#[derive(Debug, Deserialize)]
pub struct USX {
    pub(crate) r#type: String,
    pub(crate) version: String,
    pub(crate) content: String,
}
