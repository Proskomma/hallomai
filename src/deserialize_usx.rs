#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use quick_xml::events::Event;
use quick_xml::Reader;
use tempfile::NamedTempFile;
use crate::model_traits::AosjModel;



/// # Reads the USX file and reconstructs it into an AosjModel.
///
/// This function processes a USX file, parsing its
/// content and reconstructing it into a model that implements the `AosjModel`
/// trait. It handles different types of XML events such as start tags, end tags,
/// empty elements, and text nodes.
pub fn deserialize_from_file_path_usx<T:AosjModel>(input_file_path: &str) -> String {
    let mut reader = Reader::from_file(input_file_path).expect("Unable to open file");
    reader.config_mut().trim_text(true);
    deserialize_from_file_usx::<T>(reader)
}

pub fn deserialize_from_file_str_usx<T:AosjModel>(content: String) -> String {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    write!(file, "{}", content).expect("Failed to write to temp file");
    let file_path = file.path().to_str().unwrap();
    deserialize_from_file_path_usx::<T>(file_path)
}

fn deserialize_from_file_usx<T:AosjModel>(mut reader: Reader<BufReader<File>>) -> String {
    // let mut reader = Reader::from_file(input_file_path).unwrap();
    // reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut txt = Vec::new();

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
                }

                model.parent_els().pop();
            }

            Ok(Event::Text(el)) => {
                if model.parent_els().len()>1 {
                    txt.push(el.unescape().unwrap().into_owned());
                }
            }

            Ok(Event::End(..)) => {

                let tag_name = model.parent_els().last().unwrap().tag_name.clone();

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