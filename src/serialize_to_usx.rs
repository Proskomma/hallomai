#![allow(dead_code)]

use std::fs::File;
use serde_json::Value;
use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::writer::Writer;
use std::io::BufWriter;

pub fn serialize_to_usx(usj: Value, output_file_path: &str) {
    let file = File::create(output_file_path).expect("Unable to create file");

    let mut writer = Writer::new_with_indent(BufWriter::new(file), b' ', 4);

    if let Some(version) = usj.get("version").and_then(|v| v.as_str()) {
        let usx_start = BytesStart::new("usx");
        let usx_start = usx_start.with_attributes(vec![("version", version)]);
        writer.write_event(Event::Start(usx_start)).unwrap();
    }

    if let Some(content) = usj.get("content").and_then(|c| c.as_array()) {
        for element in content {
            if let Some(obj) = element.as_object() {
                match obj.get("type").and_then(|t| t.as_str()) {
                    Some("book") => {
                        let mut book_start = BytesStart::new("book");
                        for (key, value) in obj.iter() {
                            if key != "type" && key != "content" {
                                let attr_key =
                                    if key == "marker" {
                                        "style"
                                    } else {
                                        key
                                    };
                                if let Some(attr_value) = value.as_str() {
                                    book_start.push_attribute((attr_key, attr_value));
                                }
                            }
                        }
                        writer.write_event(Event::Start(book_start)).unwrap();
                        if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                            for value in content {
                                if let Some(text) = value.as_str() {
                                    writer.write_event(Event::Text(BytesText::new(text))).unwrap();
                                }
                            }
                        }
                        writer.write_event(Event::End(BytesEnd::new("book"))).unwrap();
                    }
                    Some("para") => {
                        let mut para_start = BytesStart::new("para");
                        for (key, value) in obj.iter() {
                            if key != "type" && key != "content" {
                                let attr_key =
                                    if key == "marker" {
                                        "style"
                                    } else {
                                        key
                                    };
                                if let Some(attr_value) = value.as_str() {
                                    para_start.push_attribute((attr_key, attr_value));
                                }
                            }
                        }
                        writer.write_event(Event::Start(para_start)).unwrap();
                        if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                            for value in content {
                                writer = write_content(value, writer);
                            }
                        }
                        writer.write_event(Event::End(BytesEnd::new("para"))).unwrap();
                    }
                    Some("chapter") => {
                        let mut chapter_start = BytesStart::new("chapter");
                        for (key, value) in obj.iter() {
                            if key != "type" && key != "content" {
                                let attr_key =
                                    if key == "marker" {
                                        "style"
                                    } else {
                                        key
                                    };
                                if let Some(attr_value) = value.as_str() {
                                    chapter_start.push_attribute((attr_key, attr_value));
                                }
                            }
                        }
                        writer.write_event(Event::Empty(chapter_start)).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
    writer.write_event(Event::End(BytesEnd::new("usx"))).unwrap();
}


fn write_content(mut content: &Value, mut writer: Writer<BufWriter<File>>) -> Writer<BufWriter<File>> {
    match content {
        Value::String(text) => {
            writer.write_event(Event::Text(BytesText::new(text.as_str()))).unwrap();
        }

        Value::Object(obj) => {
            match obj.get("type").and_then(|t| t.as_str()) {
                Some("char") => {
                    let mut char_start = BytesStart::new("char");
                    for (key, value) in obj.iter() {
                        if key != "type" && key != "content" {
                            let attr_key =
                                if key == "marker" {
                                    "style"
                                } else {
                                    key
                                };
                            if let Some(attr_value) = value.as_str() {
                                char_start.push_attribute((attr_key, attr_value));
                            }
                        }
                    }
                    writer.write_event(Event::Start(char_start)).unwrap();
                    if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                        for value in content {
                            writer = write_content(value, writer);
                        }
                    }
                    writer.write_event(Event::End(BytesEnd::new("char"))).unwrap();
                }
                Some("verse") => {
                    let mut verse_start = BytesStart::new("verse");
                    for (key, value) in obj.iter() {
                        if key != "type" && key != "content" {
                            let attr_key =
                                if key == "marker" {
                                    "style"
                                } else {
                                    key
                                };
                            if let Some(attr_value) = value.as_str() {
                                verse_start.push_attribute((attr_key, attr_value));
                            }
                        }
                    }
                    writer.write_event(Event::Empty(verse_start)).unwrap();
                }
                Some("ms") => {
                    let mut ms_start = BytesStart::new("ms");

                    for (key, value) in obj.iter() {
                        if key != "type" && key != "content" {
                            let attr_key =
                                if key == "marker" {
                                    "style"
                                } else {
                                    key
                                };
                            if let Some(attr_value) = value.as_str() {
                                ms_start.push_attribute((attr_key, attr_value));
                            }
                        }
                    }

                    writer.write_event(Event::Empty(ms_start)).unwrap();
                }
                Some("note") => {
                    let mut note_start = BytesStart::new("note");
                    for (key, value) in obj.iter() {
                        if key != "type" && key != "content" {
                            let attr_key =
                                if key == "marker" {
                                    "style"
                                } else {
                                    key
                                };
                            if let Some(attr_value) = value.as_str() {
                                note_start.push_attribute((attr_key, attr_value));
                            }
                        }
                    }
                    writer.write_event(Event::Start(note_start)).unwrap();
                    if let Some(content) = obj.get("content").and_then(|c| c.as_array()) {
                        for value in content {
                            writer = write_content(value, writer);
                        }
                    }
                    writer.write_event(Event::End(BytesEnd::new("note"))).unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
    writer
}
