#![allow(dead_code)]

use serde_json::Value;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn serialize_to_usfm(usj: Value, output_file_path: &str) {
    let file = File::create(output_file_path).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    if let Some(version) = usj.get("version").and_then(|v| v.as_str()) {
        writeln!(writer, "\\usfm {}", version).unwrap();
    }

    if let Some(content) = usj.get("content").and_then(|c| c.as_array()) {
        for element in content {
            if let Some(obj) = element.as_object() {
                match obj.get("type").and_then(|t| t.as_str()) { //todo : gérer tous les attributs
                    Some("book") => {
                        write!(writer, "\\id {} ", obj.get("code").unwrap().as_str().unwrap()).unwrap();
                        if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                            for value in content {
                                if let Some(text) = value.as_str() {
                                    write!(writer, "{}", text).unwrap();
                                }
                            }
                        }
                        writeln!(writer).unwrap();
                    }
                    Some("para") => {
                        let marker = obj.get("marker").unwrap().as_str().unwrap();
                        write!(writer, r"\{}", marker).unwrap();
                        if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                            for value in content {
                                write_content(value, &mut writer);
                            }
                        }

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
}


fn write_content(content: &Value, writer: &mut BufWriter<File>) {
    let mut in_char: bool = false;
    match content {
        Value::String(text) => {
            write!(writer, " {}", text).unwrap();
        }
        Value::Object(obj) => { //todo : gérer tous les attributs
            match obj.get("type").and_then(|t| t.as_str()) {
                Some("verse") => {
                    let number = obj.get("number").unwrap().as_str().unwrap();
                    write!(writer, "\n\\v {}", number).unwrap();
                }
                Some("char") => { // todo : gérer les char imbriqués
                    let marker = obj.get("marker").unwrap().as_str().unwrap();
                    if in_char {
                        write!(writer, r"\+{} ", marker).unwrap();
                    } else {
                        write!(writer, r"\{} ", marker).unwrap();
                    }

                    if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                        for value in content {
                            write_content(value, writer);
                        }
                    }
                    if let Some(default) = obj.get("default").and_then(|d| d.as_str()) {
                        write!(writer, "|{}", default).unwrap();
                    }
                    if in_char {
                        write!(writer, r"\+{}*", marker).unwrap();
                        in_char = false;
                    } else {
                        write!(writer, r"\{}*", marker).unwrap();
                    }
                    in_char = true;
                }
                Some("ms") => {
                    let marker = obj.get("marker").unwrap().as_str().unwrap();
                    write!(writer, r"\{}", marker).unwrap();

                    write!(writer, r"\*").unwrap();
                }

                _ => {}
            }
        }
        _ => {}
    }
}