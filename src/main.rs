mod model;
mod deserialize_usx;
mod model_traits;

use std::any::Any;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display, format, Formatter};
// use std::io::Read;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;


struct Element {
    tag_name: String,
    attributes: BTreeMap<String, String>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ELEMENT<{}>, {:#?}", self.tag_name, self.attributes)
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ELEMENT<{}>, {:#?}", self.tag_name, self.attributes)
    }
}


struct AosjModelMaker {
    paras: Vec<String>,
    character_markup_stack: Vec<String>,
    root_attributes: BTreeMap<String, String>,
    current_para: String
}


fn push_element(el: BytesStart, mut parent_els: Vec<Element>) -> Vec<Element> {
    let mut attributes: BTreeMap<String, String> = BTreeMap::new();
    for att in el.attributes() {
        attributes.insert(
            String::from_utf8(att.clone().unwrap().key.local_name().as_ref().to_vec()).unwrap(),
            String::from_utf8(att.clone().unwrap().value.as_ref().to_vec()).unwrap());
    }

    parent_els.push(Element {
        tag_name: String::from_utf8(el.name().as_ref().to_vec()).unwrap(),
        attributes,
    });
    parent_els
}


fn add_root_metadata(mut root_attributes: BTreeMap<String, String>, version_value: &String) -> BTreeMap<String, String> {
    // root_attributes.insert("type".to_string(), "USJ".to_string());
    root_attributes.insert("version".to_string(), version_value.to_string());
    // println!("1 : {}", root_attributes.get("version").unwrap());
    // println!("2 : {}", root_attributes.get("version").unwrap());

    root_attributes
}

fn start_book(current_para: &mut String, code: String){
    current_para.push_str("{");
    current_para.push_str(&format!(" \"type\": \"book\", \"code\": \"{}\", \"marker\": \"id\",", code));
}

fn end_book(paras: &mut Vec<String>, current_para: &mut String, current_in_para: &mut Vec<String>) {
    current_para.push_str(&format!(" \"content\": [{}] ",current_in_para.join(",")));
    current_para.push_str("}");

    paras.push(current_para.clone());
    current_in_para.clear();
    current_para.clear();
}


fn start_new_para(current_para: &mut String, style: String) {
    current_para.push_str("{");
    current_para.push_str(&format!(" \"type\": \"para\", \"marker\": \"{}\", ", style));
}

fn end_new_para(paras: &mut Vec<String>, current_para: &mut String, current_in_para: &mut Vec<String>) {

    current_para.push_str(&format!(" \"content\": [{}] ",current_in_para.join(",")));
    current_para.push_str("}");

    paras.push(current_para.clone());
    current_in_para.clear();
    current_para.clear();

}

fn add_string_to_in_para(current_in_para: &mut Vec<String>, txt: &mut Vec<String>) {
    if txt.len() !=0 {
        current_in_para.push("\"".to_string());
        current_in_para.push(txt.join(""));
        current_in_para.push("\"".to_string());

        txt.clear();
    }
}

fn add_verse_to_in_para(current_in_para: &mut Vec<String>, number: String) {
    // current_in_para.push("{".to_string());
    current_in_para.push(format!("{{ \"type\": \"verse\", \"marker\": \"v\", \"number\": \"{}\",}}", number).to_string());

    // current_in_para.push("}".to_string());
}

fn add_chapter(paras: &mut Vec<String>, number: String) {
    // paras.push("{".to_string());
    paras.push(format!("{{ \"type\": \"chapter\", \"marker\": \"c\", \"number\": \"{}\"}}", number).to_string());

    // paras.push("}".to_string());
}


fn assemble_model(root_attributes: BTreeMap<String, String>, paras: Vec<String>, char_marker_stack: Vec<String>) -> String {
    let mut model = "".to_string();
    model += "{";
    model += &format!(" \"version\": \"{}\",", root_attributes.get("version").unwrap().to_string());
    model += &format!(" \"content\": [{}]", paras.join(","));

    model += "}";
    model
}


fn main() {
    let mut paras: Vec<String> = vec![];
    let mut char_marker_stack: Vec<String> = vec![];
    let mut root_attributes: BTreeMap<String, String> = BTreeMap::new();
    let mut current_para= String::new();
    let mut current_in_para: Vec<String> = vec![];

    // let mut metadata: BTreeMap<String, String> = BTreeMap::new();
    let mut parent_els: Vec<Element> = vec![];

    let input_file_path = "./assets/web_psa150.usx";

    let mut reader = Reader::from_file(input_file_path).unwrap();
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut txt = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(el)) => {
                add_string_to_in_para(&mut current_in_para, &mut txt);
                parent_els = push_element(el, parent_els);
                // println!("{:#?}", parent_els.last());

                let tag_name = parent_els.last().unwrap().tag_name.clone();
                if tag_name == "usx" {
                    // println!("{:#?}", parent_els.last().unwrap().attributes.get("version").unwrap().clone());
                    root_attributes = add_root_metadata(
                        root_attributes,
                        parent_els.last().unwrap().attributes.get("version").unwrap(),
                    );
                } else if tag_name == "para" {
                    start_new_para(
                        &mut current_para,
                        parent_els.last().unwrap().attributes.get("style").unwrap().to_string()
                    );

                } else if tag_name == "book" {
                    start_book(
                        &mut current_para,
                        parent_els.last().unwrap().attributes.get("code").unwrap().to_string()
                    )
                }
            }

            Ok(Event::Empty(el)) => {
                add_string_to_in_para(&mut current_in_para, &mut txt);
                parent_els = push_element(el, parent_els);
                // println!("{:#?}", parent_els.last());
                let tag_name = parent_els.last().unwrap().tag_name.clone();
                if tag_name == "verse" && !parent_els.last().unwrap().attributes.contains_key("eid") {
                    add_verse_to_in_para(&mut current_in_para, parent_els.last().unwrap().attributes.get("number").unwrap().to_string());
                };

                if tag_name == "chapter" && !parent_els.last().unwrap().attributes.contains_key("eid") {
                    add_chapter(&mut paras, parent_els.last().unwrap().attributes.get("number").unwrap().to_string());
                };

                parent_els.pop();
            }

            Ok(Event::Text(el)) => {
                // txt.pop();
                if parent_els.len()>1 {
                    txt.push(el.unescape().unwrap().into_owned());
                }
                // println!("{:#?}", txt.last());

            }

            Ok(Event::End(..)) => {
                add_string_to_in_para(&mut current_in_para, &mut txt);

                let tag_name = parent_els.last().unwrap().tag_name.clone();

                if tag_name == "para" {
                    end_new_para(
                        &mut paras,
                        &mut current_para,
                        &mut current_in_para
                    )
                } else if tag_name == "book" {
                    end_book(
                        &mut paras,
                        &mut current_para,
                        &mut current_in_para
                    )
                }
                txt.clear();
                parent_els.pop();
            }

            Ok(Event::Eof) => {
                // println!("{:#?}", parent_els);
                println!("{}", assemble_model(root_attributes, paras, char_marker_stack));
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
