#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Seek};
use quick_xml::events::Event;
use quick_xml::Reader;
use tempfile::{NamedTempFile, tempfile};
use std::io::Write;
use regex::Regex;
use crate::model_traits::AosjModel;


/// # Reads the USX file and reconstructs it into an AosjModel.
///
/// This function processes a USX file, parsing its
/// content and reconstructing it into a model that implements the `AosjModel`
/// trait. It handles different types of XML events such as start tags, end tags,
/// empty elements, and text nodes.
pub fn deserialize_from_file_usx<T:AosjModel>(input_string: String) -> String {

    let mut temp_file = tempfile().expect("Failed to create temp file");
    temp_file.write_all(input_string.as_bytes()).expect("Failed to write to temp file");
    temp_file.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek to start of file");
    let br: BufReader<File> = BufReader::new(temp_file);

    let mut reader = Reader::from_reader(br);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut txt = Vec::new();

    let mut values = Regex::new(r#"(["\\])"#).unwrap();
    let mut model = T::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(el)) => {
                model.add_string_to_in_para(
                    &mut txt
                );
                let mut attributes: BTreeMap<String, String> = BTreeMap::new();

                for att in el.attributes() {
                    attributes.insert(
                        String::from_utf8(att.clone().unwrap().key.local_name().as_ref().to_vec()).unwrap(),
                        String::from_utf8(att.clone().unwrap().value.as_ref().to_vec()).unwrap()
                    );
                }

                let tag_name = String::from_utf8(el.name().as_ref().to_vec()).unwrap();

                model.push_element(attributes, tag_name.clone());

                let current_parent = model.parent_els().clone();

                if tag_name == "usx" {
                    model.add_root_metadata(
                        current_parent.last().unwrap().attributes.get("version").unwrap(),
                    );
                } else if tag_name == "para" {
                    model.start_new_para(
                        model.get_attributes()
                    );

                } else if tag_name == "book" {
                    model.start_book(
                        model.get_attributes()
                    )
                } else if tag_name == "char" {
                    model.start_add_char_marker(
                        model.get_attributes()
                    )
                } else if tag_name == "note" {
                    model.start_add_note(
                        model.get_attributes()
                    )
                }
            }

            Ok(Event::Empty(el)) => {
                clean_whitespace(&mut txt);

                model.add_string_to_in_para(&mut txt);
                let mut attributes: BTreeMap<String, String> = BTreeMap::new();
                for att in el.attributes() {
                    attributes.insert(
                        String::from_utf8(att.clone().unwrap().key.local_name().as_ref().to_vec()).unwrap(),
                        String::from_utf8(att.clone().unwrap().value.as_ref().to_vec()).unwrap()
                    );
                }
                let tag_name = String::from_utf8(el.name().as_ref().to_vec()).unwrap();

                model.push_element(attributes, tag_name.clone());

                if tag_name == "verse" && !model.parent_els().last().unwrap().attributes.contains_key("eid") {
                    model.add_verse_to_in_para(
                        model.get_attributes()
                    );
                } else if tag_name == "chapter" && !model.parent_els().last().unwrap().attributes.contains_key("eid") {
                    model.add_chapter(
                        model.get_attributes()
                    );
                } else if tag_name == "ms" && !model.parent_els().last().unwrap().attributes.contains_key("eid") {

                    model.add_milestone(
                        model.get_attributes()
                    )
                } else if tag_name == "ms" && model.parent_els().last().unwrap().attributes.contains_key("eid") {

                    model.add_milestone(
                        model.get_attributes()
                    )
                }

                model.parent_els().pop();
            }

            Ok(Event::Text(el)) => {
                let mut value = el.unescape().unwrap().into_owned();
                let new_value = values.replace_all(value.as_str(), "\\$1");
                if model.parent_els().len()>1 {
                    txt.push(new_value.to_string());
                }
            }

            Ok(Event::End(..)) => {

                let tag_name = model.parent_els().last().unwrap().tag_name.clone();

                clean_whitespace(&mut txt);

                if tag_name == "para" {
                    model.add_string_to_in_para(
                        &mut txt
                    );
                    model.end_new_para()
                } else if tag_name == "book" {
                    model.add_string_to_in_para(
                        &mut txt
                    );
                    model.end_book()
                } else if tag_name == "char" {
                    model.end_add_char_marker(
                        &mut txt
                    )
                } else if tag_name == "note" {
                    model.end_add_note(
                        &mut txt
                    )
                }
                model.parent_els().pop();
            }

            Ok(Event::Eof) => {
                return model.assemble_model();
            }
            Err(err) => {
                panic!("Error reading XML: {}", err);
            }
            _ => {}
        }
        buf.clear();
    }
}


fn clean_whitespace(txt: &mut Vec<String>) {
    for i in txt.iter_mut() {
        *i = i.replace("\n", " ").replace("\t", " ").trim().to_string();
        *i = i.split_whitespace().collect::<Vec<&str>>().join(" ");
    }
}
