#![allow(dead_code)]
use crate::structs_model::book_object::BookObject;
use crate::structs_model::model::Content;

pub struct Root {
    pub(crate) r#type: String,
    pub(crate) version: String,
    pub(crate) content: Vec<Content>,
}

impl Root {
    pub fn add_book(&mut self, book: BookObject) {
        self.content.push(Content::Book(book));
    }

    pub fn set_version(&mut self, new_version: &str) {
        self.version = new_version.to_string();
    }
}
