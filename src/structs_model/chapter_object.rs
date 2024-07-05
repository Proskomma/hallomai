#![allow(dead_code)]
pub struct ChapterObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) number: String,
    pub(crate) altnumber: Option<String>,
    pub(crate) pubnumber: Option<String>,
}

impl ChapterObject {
    pub fn init_index(&mut self) {
        self.index = self.number.chars().collect::<Vec<_>>().len();
    }
}
