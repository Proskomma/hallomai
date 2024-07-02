use serde::Deserialize;
// use crate::model::{DataType, DataTypeTraits};

#[derive(Debug)]
pub struct USJ {
    pub(crate) r#type: String,
    pub(crate) version: String,
    pub(crate) content: String,
}
