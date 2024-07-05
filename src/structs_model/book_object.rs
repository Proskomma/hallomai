#![allow(dead_code)]
pub struct BookObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) code: String,
    pub(crate) content: Option<Vec<String>>,
}

impl BookObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for strs in self.content.iter().flatten() {
            tot_len += strs.chars().collect::<Vec<_>>().len();
        }
        self.index = tot_len;
    }

    pub fn get(&self) -> &BookObject {
        self
    }
}
