#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use crate::model_traits::AosjModel;

/// # Reads the USJ file and reconstructs it into an AosjModel.
///
/// This function processes a USJ file, parsing its content and reconstructing
/// it into a model that implements the `AosjModel` trait. It handles different
/// types of elements such as books, paragraphs, chapters, verses, characters, and notes.

fn read_content<T:AosjModel>(model: &mut T, object: &Value) {
    let mut txt: Vec<String> = Vec::new();
    match object {
        Value::String(text) => {
            txt.push(text.to_string());
        }
        Value::Object(obj) => {
            let mut attributes: BTreeMap<String, String> = BTreeMap::new();

            for (key, value) in obj.iter() {
                if key != "content" && key != "type" {
                    attributes.insert(key.to_string(), value.as_str().unwrap().parse().unwrap());
                }
            }

            let tag_name = obj.get("type").unwrap().to_string();
            model.push_element(attributes.clone(), tag_name);

            match obj.get("type").and_then(|t| t.as_str()) {
                Some("verse") => {
                    model.add_verse_to_in_para(model.get_attributes());
                    model.parent_els().pop();
                }
                Some("ms") => {
                    model.add_milestone(model.get_attributes());
                    model.parent_els().pop();
                }
                Some("char") => {
                    model.start_add_char_marker(model.get_attributes());
                    if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                        for object in contents {
                            read_content(model, object);
                        }
                    }
                    model.parent_els().pop();
                    model.end_add_char_marker(&mut txt);
                }
                Some("note") => {
                    model.start_add_note(model.get_attributes());
                    if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                        for object in contents {
                            read_content(model, object);
                        }
                    }
                    model.parent_els().pop();
                    model.end_add_note(&mut txt);
                }
                _=> {}
            }
        }
        _=> {}
    }
    model.add_string_to_in_para(&mut txt);
}


fn deserialize_from_file_usj<T:AosjModel>(json: Value) -> String {

    let mut model = T::new();

    let version = json.get("version").expect("Missing version").as_str().expect("Version should be a string");
    model.add_root_metadata(&version.to_string());

    if let Some(content) = json.get("content").and_then(|c| c.as_array()) {
        for element in content {
            if let Some(obj) = element.as_object() {

                let mut attributes: BTreeMap<String, String> = BTreeMap::new();

                for (key, value) in obj.iter() {
                    if key != "content" && key != "type" {
                        attributes.insert(key.to_string(), value.as_str().unwrap().parse().unwrap());
                    }
                }
                let tag_name = obj.get("type").unwrap().to_string();
                model.push_element(attributes, tag_name);

                match obj.get("type").and_then(|t| t.as_str()) {
                    Some("book") => {
                        model.start_book(model.get_attributes());
                        if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                            for object in contents {
                                read_content(&mut model, object);
                                model.parent_els().pop();
                            }
                        }
                        model.end_book();
                    }
                    Some("chapter") => {
                        model.add_chapter(model.get_attributes());
                    }
                    Some("para") => {
                        model.start_new_para(model.get_attributes());
                        if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                            for object in contents {
                                read_content(&mut model, object);
                            }
                        }
                        model.parent_els().pop();
                        model.end_new_para();
                    }
                    _ => {}
                }
            }

        }
    }
    model.assemble_model()
}

pub fn deserialize_from_file_path_usj<T:AosjModel>(input_file_path: &str) -> String {
    let file = File::open(input_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");
    deserialize_from_file_usj::<T>(json)
}

pub fn deserialize_from_file_str_usj<T:AosjModel>(content: String) -> String {
    let json: Value = serde_json::from_str(content.as_str()).expect("Unable to parse JSON");
    deserialize_from_file_usj::<T>(json)
}
