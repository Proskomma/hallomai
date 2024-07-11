#![allow(dead_code)]

use std::collections::BTreeMap;
use crate::aosj_string::element::Element;

/// # Trait that defines functions for adding elements to the model.
///
/// This trait provides an interface for constructing and managing the elements
/// and attributes of a document model, allowing for flexible implementations.
pub trait AosjModel {
    /// Creates a new instance of the model.
    fn new() -> Self;
    /// Pushes an element to the parent elements stack.
    fn push_element(&mut self, attributes: BTreeMap<String,String>, tag_name: String);
    fn validate_attributes(&self, tag_name: &str, attributes: &BTreeMap<String, String>) -> Result<(), String>;
    /// Retrieves a formatted string of attributes.
    fn get_attributes(&self) -> String;
    /// Adds root metadata to the model.
    fn add_root_metadata(&mut self, version_value: &String);
    /// Starts a new book with given attributes.
    fn start_book(&mut self, attributes: String);
    /// Ends the current book.
    fn end_book(&mut self);
    /// Starts a new paragraph with given attributes.
    fn start_new_para(&mut self, attributes: String);
    /// Ends the current paragraph.
    fn end_new_para(&mut self);
    /// Adds a string to the current paragraph content.
    fn add_string_to_in_para(&mut self, txt: &mut Vec<String>);
    /// Adds a new chapter with given attributes.
    fn add_chapter(&mut self, attributes: String);
    /// Adds a verse to the current paragraph.
    fn add_verse_to_in_para(&mut self, attributes: String);
    /// Adds a milestone with given attributes.
    fn add_milestone(&mut self, attributes: String);
    /// Starts adding a character marker.
    fn start_add_char_marker(&mut self, attributes: String);
    /// Ends the character marker addition.
    fn end_add_char_marker(&mut self, txt: &mut Vec<String>);
    /// Starts adding a note with given attributes.
    fn start_add_note(&mut self, attributes: String);
    /// Ends the note addition.
    fn end_add_note(&mut self, txt: &mut Vec<String>);
    /// Assembles the model into a JSON string.
    fn assemble_model(&self) -> String;
    /// Returns a mutable reference to the parent elements stack.
    fn parent_els(&mut self) -> &mut Vec<Element>;

}