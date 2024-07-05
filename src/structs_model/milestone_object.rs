#![allow(dead_code)]
use std::collections::HashMap;

pub struct MilestoneObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) who: Option<String>,
    pub(crate) additional_properties: Option<HashMap<String, String>>,
}

impl MilestoneObject {
    pub fn init_index(&mut self) {
        self.index = 0;
    }
}
