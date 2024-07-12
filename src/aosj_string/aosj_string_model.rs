#![allow(dead_code)]
use std::collections::BTreeMap;
use crate::aosj_string::aosj_string_model::element::Element;
use crate::model_traits::AosjModel;

/// # Represents the model that contains all utility objects and their types.
///
/// This struct is used to parse and model a USX (Unified Scripture XML) document,
/// providing methods to handle different elements and their attributes.
pub struct AosjStringModel {
    /// Attributes of the root element.
    pub root_attributes: BTreeMap<String, String>,
    /// Collection of paragraphs in the document.
    pub paras: Vec<String>,
    /// Current paragraph content being processed.
    pub current_para: String,
    /// Stack of paragraph content vectors.
    pub stack_in_paras: Vec<Vec<String>>,
    /// Stack for character markers.
    pub char_marker_stack: Vec<String>,
    /// Stack for notes.
    pub note_stack: Vec<String>,
    /// Stack of parent elements.
    pub parent_els: Vec<Element>
}

/// # We implement all the functions of the trait for the above model
impl AosjModel for AosjStringModel {
    /// Creates a new instance of the model.
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
    /// Pushes an element to the parent elements stack.
    fn push_element(&mut self, attributes: BTreeMap<String,String>, tag_name: String) {
        self.parent_els.push(Element {
            tag_name,
            attributes,
        });
    }


    fn validate_attributes(&self, tag_name: &str, attributes: &BTreeMap<String, String>) -> Result<(), String> {
        match tag_name {
            "para" => {
                if !attributes.contains_key("type") {
                    return Err("Missing required attribute 'type'".to_string());
                }
                if !attributes.contains_key("marker") {
                    return Err("Missing required attribute 'marker'".to_string());
                }
                for key in attributes.keys() {
                    if key != "type" && key != "content" && key != "marker" && key != "sid" {
                        return Err(format!("Unexpected attribute '{}' in 'para'", key));
                    }
                }
            },
            "book" => {
                if !attributes.contains_key("type") {
                    return Err("Missing required attribute 'type'".to_string());
                }
                if !attributes.contains_key("marker") {
                    return Err("Missing required attribute 'marker'".to_string());
                }
                if !attributes.contains_key("code") {
                    return Err("Missing required attribute 'code'".to_string());
                }
                for key in attributes.keys() {
                    if key != "type" && key != "content" && key != "marker" && key != "code" {
                        return Err(format!("Unexpected attribute '{}' in 'para'", key));
                    }
                }
            }
            "chapter" => {
                if !attributes.contains_key("type") {
                    return Err("Missing required attribute 'type'".to_string());
                }
                if !attributes.contains_key("marker") {
                    return Err("Missing required attribute 'marker'".to_string());
                }
                if !attributes.contains_key("number") {
                    return Err("Missing required attribute 'number'".to_string());
                }
                for key in attributes.keys() {
                    if key != "type" && key != "marker" && key != "number" && key != "sid" && key != "altnumber" && key != "pubnumber" {
                        return Err(format!("Unexpected attribute '{}' in 'para'", key));
                    }
                }
            }
            "char" => {
                if !attributes.contains_key("type") {
                    return Err("Missing required attribute 'type'".to_string());
                }
                if !attributes.contains_key("marker") {
                    return Err("Missing required attribute 'marker'".to_string());
                }
                for key in attributes.keys() {
                    if key != "type" && key != "content" && key != "marker" && key != "link_id" && key != "link_href" && key != "link_id" {
                        return Err(format!("Unexpected attribute '{}' in 'para'", key));
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }



    /// Retrieves a formatted string of attributes.
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


    /// Adds root metadata to the model.
    fn add_root_metadata(&mut self, version_value: &String) {
        self.root_attributes.insert("version".to_string(), version_value.to_string());
    }
    /// Starts a new book with given attributes.
    fn start_book(&mut self, attributes: String){
        self.stack_in_paras.push(vec![format!("{{ \"type\": \"book\", {}, \"content\": [", attributes)]);
    }
    /// Ends the current book.
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
    /// Starts a new paragraph with given attributes.
    fn start_new_para(&mut self, attributes: String) {
        self.stack_in_paras.push(vec![format!("{{ \"type\": \"para\",{}, \"content\": [ ", attributes)]);
    }
    /// Ends the current paragraph.
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
    /// Adds a string to the current paragraph content
    fn add_string_to_in_para(&mut self, txt: &mut Vec<String>) {
        if txt.len() !=0 {

            let mut last = self.stack_in_paras.pop().unwrap();
            last.push(format!("\"{}\",", txt.join("")).to_string());
            // println!("{:#?}", last);
            self.stack_in_paras.push(last);

            txt.clear();
        }
    }
    /// Adds a new chapter with given attributes.
    fn add_chapter(&mut self, attributes: String) {
        self.paras.push(format!("{{ \"type\": \"chapter\", {} }}", attributes).to_string());
    }
    /// Adds a verse to the current paragraph.
    fn add_verse_to_in_para(&mut self, attributes: String) {
        let mut last = self.stack_in_paras.pop().unwrap();
        last.push(format!("{{ \"type\": \"verse\", {} }},", attributes).to_string());
        self.stack_in_paras.push(last);

    }
    /// Adds a milestone with given attributes.
    fn add_milestone(&mut self, attributes: String) {
        let mut last = self.stack_in_paras.pop().unwrap();
        last.push(format!("{{ \"type\": \"ms\", {} }},", attributes).to_string());
        self.stack_in_paras.push(last);

    }
    /// Starts adding a character marker.
    fn start_add_char_marker(&mut self, attributes: String) {
        let mut current_char:Vec<String> = Vec::new();
        current_char.push(format!("{{ \"type\": \"char\", {}, \"content\": [", attributes).to_string());
        self.stack_in_paras.push(current_char);
    }
    /// Ends the character marker addition.
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
    /// Starts adding a note with given attributes.
    fn start_add_note(&mut self, attributes: String) {
        self.stack_in_paras.push(vec![format!("{{ \"type\": \"note\", {}, \"content\": [", attributes).to_string()]);
    }
    /// Ends the note addition.
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


    /// Assembles the model into a JSON string.
    fn assemble_model(&self) -> String {
        let mut model = "".to_string();
        model += "{";
        model += &format!(" \"version\": \"{}\",", self.root_attributes.get("version").unwrap().to_string());
        model += &format!(" \"content\": [{}]", self.paras.join(","));

        model += "}";
        model
    }

    /// Returns a mutable reference to the parent elements stack.
    fn parent_els(&mut self) -> &mut Vec<Element> {
        &mut self.parent_els
    }

}
