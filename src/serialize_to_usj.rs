#![allow(dead_code)]
use std::fs::File;
use std::io::Write;
use serde_json::{to_string_pretty, Value};


pub fn serialize_to_usj(mut content: String) -> Value {
    let mut json_value: Value = serde_json::from_str(content.as_mut_str()).expect("Invalid JSON string");
    if let Value::Object(ref mut map) = json_value {
        map.insert("type".to_string(), Value::String("USJ".to_string()));
    }
    let json_with_sid = add_sid_to_usj(json_value);
    // let formatted_json = to_string_pretty(&json_with_sid).expect("Failed to format JSON");
    // let mut file = File::create(output_file_path).expect("Unable to create file");
    // file.write_all(formatted_json.as_bytes()).expect("Unable to write data");
    json_with_sid
}

fn add_sid_to_usj(mut usj_json: Value) -> Value {

    let mut book_code = String::new();
    let mut chapter_number = String::new();

    if let Some(content_array) = usj_json.get_mut("content").and_then(|c| c.as_array_mut()) {
        for content in content_array.iter_mut() {
            if let Some(content_obj) = content.as_object_mut() {
                match content_obj.get("type").and_then(|t| t.as_str()) {
                    Some("book") => {

                        if let Some(code) = content_obj.get("code").and_then(|c| c.as_str()) {
                            book_code = code.to_string();
                        }
                    },
                    Some("chapter") => {

                        if let Some(number) = content_obj.get("number").and_then(|n| n.as_str()) {
                            chapter_number = number.to_string();
                            let sid = format!("{} {}", book_code, chapter_number);
                            content_obj.insert("sid".to_string(), Value::String(sid));
                        }
                    },
                    Some("para") => {
                        if let Some(para_content) = content_obj.get_mut("content").and_then(|c| c.as_array_mut()) {
                            for para_item in para_content.iter_mut() {
                                if let Some(para_obj) = para_item.as_object_mut() {
                                    if para_obj.get("type").and_then(|t| t.as_str()) == Some("verse") {
                                        if let Some(number) = para_obj.get("number").and_then(|n| n.as_str()) {
                                            let sid = format!("{} {}:{}", book_code, chapter_number, number);
                                            para_obj.insert("sid".to_string(), Value::String(sid));
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
    }
    usj_json
}
