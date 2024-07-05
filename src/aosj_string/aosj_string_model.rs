#![allow(dead_code)]
use std::collections::BTreeMap;
use quick_xml::events::BytesStart;
use crate::aosj_string::element::Element;
use crate::model_traits::AosjModel;

pub struct AosjStringModel {
    pub root_attributes: BTreeMap<String, String>,
    pub paras: Vec<String>,
    pub current_para: String,
    pub stack_in_paras: Vec<Vec<String>>,
    pub char_marker_stack: Vec<String>,
    pub note_stack: Vec<String>,
    pub parent_els: Vec<Element>
}

impl AosjModel for AosjStringModel {

    fn new() -> Self {
        AosjStringModel {
            root_attributes: BTreeMap::new(),
            paras: Vec::new(),
            current_para: String::new(),
            stack_in_paras: Vec::new(),
            char_marker_stack: Vec::new(),
            note_stack: Vec::new(),
            parent_els: Vec::new()
        }
    }

    fn push_element(&mut self, el: BytesStart) {
        let mut attributes: BTreeMap<String, String> = BTreeMap::new();
        for att in el.attributes() {
            attributes.insert(
                String::from_utf8(att.clone().unwrap().key.local_name().as_ref().to_vec()).unwrap(),
                String::from_utf8(att.clone().unwrap().value.as_ref().to_vec()).unwrap());
        }

        self.parent_els.push(Element {
            tag_name: String::from_utf8(el.name().as_ref().to_vec()).unwrap(),
            attributes,
        });
    }



    fn get_attributes(&self) -> String {
        let mut attributes = Vec::new();
        if let Some(element) = self.parent_els.last() {
            for (key, value) in &element.attributes {
                if key != "sid" && key != "vid" && key != "eid" {
                    let mut good_key = key.to_string();
                    if key == "style" {
                        good_key = "marker".to_string();
                    }
                    attributes.push(format!("\"{}\": \"{}\"", good_key, value));
                }
            }
        }
        attributes.join(", ")
    }



    fn add_root_metadata(&mut self, version_value: &String) {
        self.root_attributes.insert("version".to_string(), version_value.to_string());
    }

    fn start_book(&mut self, attributes: String){
        self.stack_in_paras.push(vec![format!("{{ \"type\": \"book\", {}, \"content\": [", attributes)]);
    }

    fn end_book(&mut self) {
        let mut last = self.stack_in_paras.pop().unwrap();
        let mut last_of_last = last.pop().unwrap();
        let ultimate = last_of_last.pop().unwrap().to_string();
        if ultimate != "," {
            last_of_last.push_str(ultimate.as_str());
        }
        last.push(last_of_last);
        last.push(format!(" ] }}").to_string());
        self.stack_in_paras.push(last);

        self.paras.push(self.stack_in_paras.pop().unwrap().join(" "));
        self.stack_in_paras.clear();
    }

    fn start_new_para(&mut self, attributes: String) {
        self.stack_in_paras.push(vec![format!("{{ \"type\": \"para\",{}, \"content\": [ ", attributes)]);
    }

    fn end_new_para(&mut self) {
        let mut last = self.stack_in_paras.pop().unwrap();
        let mut last_of_last = last.pop().unwrap();
        let ultimate = last_of_last.pop().unwrap().to_string();
        if ultimate != "," {
            last_of_last.push_str(ultimate.as_str());
        }
        last.push(last_of_last);
        last.push(format!(" ] }}").to_string());
        self.stack_in_paras.push(last);

        self.paras.push(self.stack_in_paras.pop().unwrap().join(" "));
        self.stack_in_paras.clear();
    }

    fn add_string_to_in_para(&mut self, txt: &mut Vec<String>) {
        if txt.len() !=0 {

            let mut last = self.stack_in_paras.pop().unwrap();
            last.push(format!("\"{}\",", txt.join("")).to_string());
            // println!("{:#?}", last);
            self.stack_in_paras.push(last);

            txt.clear();
        }
    }

    fn add_chapter(&mut self, attributes: String) {
        self.paras.push(format!("{{ \"type\": \"chapter\", {} }}", attributes).to_string());
    }

    fn add_verse_to_in_para(&mut self, attributes: String) {
        let mut last = self.stack_in_paras.pop().unwrap();
        last.push(format!("{{ \"type\": \"verse\", {} }},", attributes).to_string());
        self.stack_in_paras.push(last);

    }

    fn add_milestone(&mut self, attributes: String) {
        let mut last = self.stack_in_paras.pop().unwrap();
        last.push(format!("{{ \"type\": \"ms\", {} }},", attributes).to_string());
        self.stack_in_paras.push(last);

    }

    fn start_add_char_marker(&mut self, attributes: String) {
        let mut current_char:Vec<String> = Vec::new();
        current_char.push(format!("{{ \"type\": \"char\", {}, \"content\": [", attributes).to_string());
        self.stack_in_paras.push(current_char);
    }

    fn end_add_char_marker(&mut self, txt: &mut Vec<String>) {
        if !txt.is_empty() {
            let mut last = self.stack_in_paras.pop().unwrap();
            last.push(format!("\"{}\"", txt.join("")));
            self.stack_in_paras.push(last);

            txt.clear();

        }
        let mut last = self.stack_in_paras.pop().unwrap();
        let mut last_of_last = last.pop().unwrap();
        let ultimate = last_of_last.pop().unwrap().to_string();
        if ultimate != "," {
            last_of_last.push_str(ultimate.as_str());
        }
        last.push(last_of_last);
        last.push(format!(" ] }},").to_string());
        self.stack_in_paras.push(last);


        let element = self.stack_in_paras.pop().unwrap();
        if !element.is_empty() {
            let mut last = self.stack_in_paras.pop().unwrap();
            last.push(element.join(" "));
            self.stack_in_paras.push(last);
        }
    }

    fn start_add_note(&mut self, attributes: String) {
        self.stack_in_paras.push(vec![format!("{{ \"type\": \"note\", {}, \"content\": [", attributes).to_string()]);
    }

    fn end_add_note(&mut self, txt: &mut Vec<String>) {
        if !txt.is_empty() {
            let mut last = self.stack_in_paras.pop().unwrap();
            last.push(format!("\"{}\"", txt.join("")));
            self.stack_in_paras.push(last);

            txt.clear();

        }
        let mut last = self.stack_in_paras.pop().unwrap();
        let mut last_of_last = last.pop().unwrap();
        let ultimate = last_of_last.pop().unwrap().to_string();
        if ultimate != "," {
            last_of_last.push_str(ultimate.as_str());
        }
        last.push(last_of_last);
        last.push(format!(" ] }},").to_string());
        self.stack_in_paras.push(last);


        let element = self.stack_in_paras.pop().unwrap();
        if !element.is_empty() {
            let mut last = self.stack_in_paras.pop().unwrap();
            last.push(element.join(" "));
            self.stack_in_paras.push(last);
        }
    }



    fn assemble_model(&self) -> String {
        let mut model = "".to_string();
        model += "{";
        model += &format!(" \"version\": \"{}\",", self.root_attributes.get("version").unwrap().to_string());
        model += &format!(" \"content\": [{}]", self.paras.join(","));

        model += "}";
        model
    }


    fn parent_els(&mut self) -> &mut Vec<Element> {
        &mut self.parent_els
    }

}
