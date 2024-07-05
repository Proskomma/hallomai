#![allow(dead_code)]

use quick_xml::events::Event;
use quick_xml::Reader;
use crate::model_traits::AosjModel;


pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) {
    let mut reader = Reader::from_file(input_file_path).unwrap();
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut txt = Vec::new();

    let mut model = T::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(el)) => {
                model.add_string_to_in_para(
                    &mut txt
                );
                model.push_element(el);
                let current_parent = model.parent_els().clone();

                let tag_name = model.parent_els().last().unwrap().tag_name.clone();
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
                model.push_element(el);
                let tag_name = model.parent_els().last().unwrap().tag_name.clone();

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
                println!("{}", model.assemble_model());
                break;
            }
            Err(err) => {
                println!("{}", err);
                break;
            }
            _ => {}
        }
        buf.clear();
    }
}
