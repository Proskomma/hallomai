use serde::Deserialize;
use crate::model::{DataType, DataTypeTraits};

fn default_type() -> String {
    "usx".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct USX {
    #[serde(default = "default_type")]
    pub(crate) r#type: String,
    pub(crate) version: String,
    #[serde(alias = "$value")]
    pub(crate) content: String,
}
