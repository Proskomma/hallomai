#![allow(dead_code)]
pub struct VerseObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) number: String,
    pub(crate) altnumber: Option<String>,
    pub(crate) pubnumber: Option<String>,
}
impl VerseObject {
    pub fn init_index(&mut self) {
        let num = &self.number;
        self.index = num.chars().collect::<Vec<_>>().len();
    }

    pub fn get_index(self) -> usize {
        self.index
    }
}
