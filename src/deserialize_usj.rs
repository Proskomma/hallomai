#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use crate::model_traits::AosjModel;

/// # Reads the USJ file and reconstructs it into an AosjModel.
///
/// These functions process a USJ file, parsing its
/// content and reconstructing it into a model that implements the `AosjModel`
/// trait. It handles different types of XML events such as start tags, end tags,
/// empty elements, and text nodes.

fn read_content<T:AosjModel>(model: &mut T, object: &Value) {
    let mut txt: Vec<String> = Vec::new();
    match object {
        Value::String(text) => {
            txt.push(text.to_string());
        }
        Value::Object(obj) => {
            match obj.get("type").and_then(|t| t.as_str()) {
                Some("verse") => {
                    let marker = obj.get("marker").expect("Missing marker").as_str().expect("Marker should be a string");
                    let number = obj.get("number").expect("Missing number").as_str().expect("Number should be a string");
                    model.add_verse_to_in_para(format!("\"marker\": \"{}\", \"number\": \"{}\"", marker, number));
                }
                Some("char") => {
                    let marker = obj.get("marker").expect("Missing marker").as_str().expect("Marker should be a string");
                    model.start_add_char_marker(format!("\"marker\": \"{}\"", marker));
                    if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                        for object in contents {
                            read_content(model, object);
                        }
                    }
                    model.end_add_char_marker(&mut txt);
                }
                Some("note") => {
                    let marker = obj.get("marker").expect("Missing marker").as_str().expect("Marker should be a string");
                    model.start_add_note(format!("\"marker\": \"{}\"", marker));
                    if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                        for object in contents {
                            read_content(model, object);
                        }
                    }
                    model.end_add_note(&mut txt);
                }
                _=> {}
            }
        }
        _=> {}
    }
    model.add_string_to_in_para(&mut txt);
}

pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) {

    let file = File::open(input_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");

    let mut model = T::new();

    let version = json.get("version").expect("Missing version").as_str().expect("Version should be a string");
    model.add_root_metadata(&version.to_string());

    if let Some(content) = json.get("content").and_then(|c| c.as_array()) {
        for element in content {
            if let Some(obj) = element.as_object() {
                match obj.get("type").and_then(|t| t.as_str()) {
                    Some("book") => {
                        let marker = obj.get("marker").expect("Missing marker").as_str().expect("Marker should be a string");
                        let code = obj.get("code").expect("Missing code").as_str().expect("Code should be a string");
                        model.start_book(format!("\"marker\": \"{}\", \"code\": \"{}\"", marker, code));
                        if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                            for object in contents {
                                read_content(&mut model, object);
                            }
                        }
                        model.end_book();
                    }
                    Some("chapter") => {
                        let marker = obj.get("marker").expect("Missing marker").as_str().expect("Marker should be a string");
                        let number = obj.get("number").expect("Missing number").as_str().expect("Number should be a string");
                        model.add_chapter(format!("\"marker\": \"{}\", \"number\": \"{}\"", marker, number));
                    }
                    Some("para") => {
                        let marker = obj.get("marker").expect("Missing marker").as_str().expect("Marker should be a string");
                        model.start_new_para(format!("\"marker\": \"{}\"", marker));
                        if let Some(contents) = obj.get("content").and_then(|c| c.as_array()) {
                            for object in contents {
                                read_content(&mut model, object);
                            }
                        }
                        model.end_new_para();
                    }
                    _ => {}
                }
            }

        }
    }
    println!("{}", model.assemble_model());
}
