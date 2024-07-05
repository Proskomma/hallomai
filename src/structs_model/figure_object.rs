#![allow(dead_code)]
pub struct FigureObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<String>>,
    pub(crate) file: Option<String>,
    pub(crate) size: Option<String>,
    pub(crate) r#ref: Option<String>,
}

impl FigureObject {
    pub fn init_index(&mut self) {
        self.index = 0;
    }
}
