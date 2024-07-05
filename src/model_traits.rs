#![allow(dead_code)]

use quick_xml::events::BytesStart;
use crate::aosj_string::element::Element;

pub trait AosjModel {
    fn new() -> Self;
    fn push_element(&mut self, el: BytesStart);
    fn get_attributes(&self) -> String;
    fn add_root_metadata(&mut self, version_value: &String);
    fn start_book(&mut self, attributes: String);
    fn end_book(&mut self);
    fn start_new_para(&mut self, attributes: String);
    fn end_new_para(&mut self);
    fn add_string_to_in_para(&mut self, txt: &mut Vec<String>);
    fn add_chapter(&mut self, attributes: String);
    fn add_verse_to_in_para(&mut self, attributes: String);
    fn add_milestone(&mut self, attributes: String);
    fn start_add_char_marker(&mut self, attributes: String);
    fn end_add_char_marker(&mut self, txt: &mut Vec<String>);
    fn start_add_note(&mut self, attributes: String);
    fn end_add_note(&mut self, txt: &mut Vec<String>);
    fn assemble_model(&self) -> String;
    fn parent_els(&mut self) -> &mut Vec<Element>;

}