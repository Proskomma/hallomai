// #![allow(dead_code)]
//
// use std::collections::BTreeMap;
// use crate::aosj_string::element::Element;
// use crate::model_traits::AosjModel;
// use crate::structs_model::model;
//
// impl AosjModel for model::Model {
//     fn new() -> Self {
//         todo!()
//     }
//
//     fn push_element(&mut self, attributes: BTreeMap<String, String>, tag_name: String) {
//         todo!()
//     }
//
//     fn get_attributes(&self) -> String {
//         todo!()
//     }
//
//     fn add_root_metadata(&mut self, version_value: &String) {
//         todo!()
//     }
//
//     fn start_book(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn end_book(&mut self) {
//         todo!()
//     }
//
//     fn start_new_para(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn end_new_para(&mut self) {
//         todo!()
//     }
//
//     fn add_string_to_in_para(&mut self, txt: &mut Vec<String>) {
//         todo!()
//     }
//
//     fn add_chapter(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn add_verse_to_in_para(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn add_milestone(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn start_add_char_marker(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn end_add_char_marker(&mut self, txt: &mut Vec<String>) {
//         todo!()
//     }
//
//     fn start_add_note(&mut self, attributes: String) {
//         todo!()
//     }
//
//     fn end_add_note(&mut self, txt: &mut Vec<String>) {
//         todo!()
//     }
//
//     fn assemble_model(&self) -> String {
//         todo!()
//     }
//
//     fn parent_els(&mut self) -> &mut Vec<Element> {
//         todo!()
//     }
// }
