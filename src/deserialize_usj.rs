use serde::Deserialize;
// use crate::model::{DataType, DataTypeTraits};

#[derive(Debug, Deserialize)]
pub struct USJ {
    pub(crate) r#type: String,
    pub(crate) version: String,
    pub(crate) content: String,
}
