#![allow(dead_code)]

use serde_json::Value;
use std::io::{BufWriter, Write};

pub fn serialize_to_e_bible(usj: Value) -> String {
    let mut writer = BufWriter::new(Vec::new());

    if let Some(version) = usj.get("version").and_then(|v| v.as_str()) {
        writeln!(writer, "\\usfm {}", version).unwrap();
    }

    if let Some(content) = usj.get("content").and_then(|c| c.as_array()) {
        for element in content {
            if let Some(obj) = element.as_object() {
                match obj.get("type").and_then(|t| t.as_str()) {
                    Some("book") => {

                        writeln!(writer).unwrap();
                    }
                    Some("para") => {
                        
                        writeln!(writer).unwrap();
                    }
                    Some("chapter") => {
                        let number = obj.get("number").unwrap().as_str().unwrap();
                        writeln!(writer, r"\c {}", number).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
    let buffer = writer.into_inner().expect("Failed to retrieve buffer");

    let output_string = String::from_utf8(buffer).expect("Failed to convert buffer to string");
    output_string
}


fn write_content(content: &Value, writer: &mut BufWriter<Vec<u8>>, in_char: bool) {
    match content {
        Value::String(text) => {
            write!(writer, "{}", text).unwrap();
        }
        Value::Object(obj) => {
            match obj.get("type").and_then(|t| t.as_str()) {
                Some("verse") => {
                    let number = obj.get("number").unwrap().as_str().unwrap();
                    write!(writer, "\n\\v {} ", number).unwrap();
                }
                Some("char") => {
                    let marker = obj.get("marker").unwrap().as_str().unwrap();
                    let mut is_nested: String = "".to_string();
                    if in_char {
                        is_nested = "+".to_string();
                        write!(writer, r"\{}{} ", is_nested, marker).unwrap();
                    } else {
                        write!(writer, r"\{} ", marker).unwrap();
                    }

                    if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                        for value in content {
                            write_content(value, writer, true);
                        }
                    }

                    if obj.get("marker").and_then(|d| d.as_str()) == Some("w") {
                        if let Some(default) = obj.get("default").and_then(|d| d.as_str()) {
                            write!(writer, "|{} ", default).unwrap();
                        } else {
                            write!(writer, " | ").unwrap();
                            for (key, value) in obj.iter() {
                                if key != "type" && key != "marker" && key != "content" && key != "default" {
                                    write!(writer, "{}={}", key, value).unwrap();
                                }
                            }
                        }
                    }

                    for (key, value) in obj.iter() {
                        if key != "type" && key != "marker" && key != "content" && key != "default"{
                            write!(writer, r"{}={} ", key, value).unwrap();
                        }
                    }
                    if !is_nested.is_empty() {
                        write!(writer, r"\{}{}*", is_nested, marker).unwrap();
                        is_nested.clear();
                    } else {
                        write!(writer, r"\{}*", marker).unwrap();
                    }
                }
                Some("ms") => {
                    let marker = obj.get("marker").unwrap().as_str().unwrap();
                    write!(writer, r"\{}", marker).unwrap();

                    let mut first_key_value = true;
                    for (key, value) in obj.iter() {
                        if key != "type" && key != "marker" && key != "content" {
                            if first_key_value {
                                write!(writer, " | ").unwrap();
                                first_key_value = false;
                            }
                            write!(writer, r"{}={} ", key, value).unwrap();
                        }
                    }
                    write!(writer, r"\*").unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
}