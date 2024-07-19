#[allow(dead_code)]
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use regex::Regex;

// use rust_usfm_parser;
// use rust_usfm_parser::USFMParser;
// use tree_sitter;
// use tree_sitter::Node;

use crate::model_traits::AosjModel;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Chapter,
    PubChapter,
    Verses,
    Attribute,
    DefaultAttribute,
    EmptyMilestone,
    StartMilestoneTag,
    EndMilestoneMarker,
    EndTag,
    StartTag,
    BareSlash,
    Eol,
    NoBreakSpace,
    SoftLineBreak,
    WordLike,
    LineSpace,
    Punctuation,
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct TokenEvent {
    token_type: TokenType,
    value: String,
}

pub struct LexingRegex {
    category: &'static str,
    name: TokenType,
    regex: Regex,
}

pub fn lexing_regex(usfm_string: &str) -> Vec<TokenEvent>{
    let lexing_regexes = vec![
        LexingRegex {
            category: "chapter",
            name: TokenType::Chapter,
            regex: Regex::new(r"([\r\n]*\\c[ \t]+(\d+)[ \t\r\n]*)").unwrap(),
        },
        LexingRegex {
            category: "pubchapter",
            name: TokenType::PubChapter,
            regex: Regex::new(r"([\r\n]*\\cp[ \t]+([^\r\n]+)[ \t\r\n]*)").unwrap(),
        },
        LexingRegex {
            category: "verses",
            name: TokenType::Verses,
            regex: Regex::new(r"(\\v[ \t]+([\d\-]+)[ \t\r\n]*)").unwrap(),
        },
        LexingRegex {
            category: "attribute",
            name: TokenType::Attribute,
            regex: Regex::new(r#"([ \t]*\|?[ \t]*([A-Za-z0-9\-]+)="([^"]*)"[ \t]?)"#).unwrap(),
        },
        LexingRegex {
            category: "defaultAttribute",
            name: TokenType::DefaultAttribute,
            regex: Regex::new(r"([ \t]*\|[ \t]*([^\|\\]*))").unwrap(),
        },
        LexingRegex {
            category: "emptyMilestone",
            name: TokenType::EmptyMilestone,
            regex: Regex::new(r"(\\([a-z1-9]+)\\[*])").unwrap(),
        },
        LexingRegex {
            category: "startMilestoneTag",
            name: TokenType::StartMilestoneTag,
            regex: Regex::new(r"(\\([a-z1-9]+)-([se]))").unwrap(),
        },
        LexingRegex {
            category: "endMilestoneMarker",
            name: TokenType::EndMilestoneMarker,
            regex: Regex::new(r"(\\([*]))").unwrap(),
        },
        LexingRegex {
            category: "endTag",
            name: TokenType::EndTag,
            regex: Regex::new(r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[*])").unwrap(),
        },
        LexingRegex {
            category: "startTag",
            name: TokenType::StartTag,
            regex: Regex::new(r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[ \t]?)").unwrap(),
        },
        LexingRegex {
            category: "bareSlash",
            name: TokenType::BareSlash,
            regex: Regex::new(r"(\\)").unwrap(),
        },
        LexingRegex {
            category: "eol",
            name: TokenType::Eol,
            regex: Regex::new(r#"([ \t]*[\r\n]+[ \t]*)"#).unwrap(),
        },
        LexingRegex {
            category: "noBreakSpace",
            name: TokenType::NoBreakSpace,
            regex: Regex::new(r"~").unwrap(),
        },
        LexingRegex {
            category: "softLineBreak",
            name: TokenType::SoftLineBreak,
            regex: Regex::new(r"//").unwrap(),
        },
        LexingRegex {
            category: "wordLike",
            name: TokenType::WordLike,
            regex: Regex::new(r"([\p{Letter}\p{Number}\p{Mark}\u2060]{1,127})").unwrap(),
        },
        LexingRegex {
            category: "lineSpace",
            name: TokenType::LineSpace,
            regex: Regex::new(r"([\p{Separator}\t]{1,127})").unwrap(),
        },
        LexingRegex {
            category: "punctuation",
            name: TokenType::Punctuation,
            regex: Regex::new(r"([\p{Punctuation}\p{Math_Symbol}\p{Currency_Symbol}\p{Modifier_Symbol}\p{Other_Symbol}])").unwrap(),
        },
        LexingRegex {
            category: "unknown",
            name: TokenType::Unknown,
            regex: Regex::new(r"(.)").unwrap(),
        },
    ];

    // Creating the main regex by combining all individual regexes
    let main_regex = Regex::new(
        &lexing_regexes.iter()
            .map(|lr| lr.regex.as_str())
            .collect::<Vec<_>>()
            .join("|")
    ).unwrap();

    // Example usage

    let mut events: Vec<TokenEvent> = Vec::new();

    for cap in main_regex.captures_iter(usfm_string) {
        let cap_str = cap.get(0).unwrap().as_str().to_string();
        for lex in &lexing_regexes {
            if let Some(m) = lex.regex.captures(&cap_str) {
                events.push(TokenEvent {
                    token_type: lex.name.clone(),
                    value: m.get(0).unwrap().as_str().to_string(),
                });
                break;
            }
        }
    }

    // println!("{:#?}", events[0].token_type);
    // Example of iterating through events
    // for event in &events {
    //     println!("{:?}: {:?}", event.token_type, event.value);
    // }

    events
}


pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) -> String {

    let mut file = File::open(input_file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let events = lexing_regex(content.as_str());
    // println!("{:#?}",events);

    let mut model = T::new();

    let mut attributes: BTreeMap<String, String> = BTreeMap::new();
    let number = Regex::new(r"\d+").unwrap();

    let exclude_types = vec![TokenType::WordLike, TokenType::LineSpace, TokenType::Punctuation];
    let mut txt: Vec<String> = Vec::new();
    let mut current_state: String = String::new();
    let mut in_para: bool = false;


    for event in &events {
        if !exclude_types.contains(&event.token_type) {
            let mut marker = event.value.trim().replace("\\", "");


            match (&event.token_type, marker.as_str(), current_state.as_str()) {
                (TokenType::StartTag, "usfm", _) => {
                    current_state = "usfm".to_string();
                }
                (TokenType::StartTag, "id", _ ) => {
                    current_state = "book".to_string();
                    attributes.insert("marker".to_string(), marker.clone());
                }
                (TokenType::StartTag, "bd", _) => {
                    current_state = "char".to_string();

                    let tag_name = "char".to_string();
                    attributes.insert("marker".to_string(), marker);
                    model.push_element(attributes.clone(), tag_name);
                    model.start_add_char_marker(
                        model.get_attributes()
                    );
                    attributes.clear();
                }
                (TokenType::StartTag, "qs", _) => {
                    current_state = "char".to_string();

                    let tag_name = "char".to_string();
                    attributes.insert("marker".to_string(), marker);
                    model.push_element(attributes.clone(), tag_name);
                    model.start_add_char_marker(
                        model.get_attributes()
                    );
                    attributes.clear();
                }
                (TokenType::StartTag, "w", _) => {
                    current_state = "char".to_string();

                    let tag_name = "char".to_string();
                    attributes.insert("marker".to_string(), marker);
                    model.push_element(attributes.clone(), tag_name);
                    model.start_add_char_marker(
                        model.get_attributes()
                    );
                    attributes.clear();
                }
                (TokenType::StartTag, "+it", _) => {
                    current_state = "char".to_string();

                    let tag_name = "char".to_string();
                    attributes.insert("marker".to_string(), marker);
                    model.push_element(attributes.clone(), tag_name);
                    model.start_add_char_marker(
                        model.get_attributes()
                    );
                    attributes.clear();
                }
                (TokenType::StartTag, _, _) => {
                    if in_para == true  && !txt.is_empty() {
                        model.add_string_to_in_para(&mut txt);
                        model.end_new_para();
                        txt.clear();
                    } else {
                        let tag_name = "para".to_string();
                        model.push_element(attributes.clone(), tag_name);
                        model.start_new_para(model.get_attributes());
                        in_para = true;
                    }
                    current_state = "para".to_string();
                    attributes.insert("marker".to_string(), marker.clone());

                }
                (TokenType::Chapter, _, _) => {
                    attributes.clear();
                    current_state ="c".to_string();
                    let tag_name = "chapter".to_string();
                    attributes.insert("marker".to_string(), "c".to_string());

                    if let Some(num) = number.captures(event.value.as_str()) {
                        attributes.insert("number".to_string(), num[0].to_string());
                    }

                    model.push_element(attributes.clone(), tag_name);
                    model.add_chapter(model.get_attributes());
                    attributes.clear();
                }
                (TokenType::Verses, _, _) => {
                    attributes.clear();
                    current_state ="verse".to_string();
                    let tag_name = "verse".to_string();
                    attributes.insert("marker".to_string(), "v".to_string());
                    if let Some(num) = number.captures(event.value.as_str()) {
                        attributes.insert("number".to_string(), num[0].to_string());
                    }
                    model.push_element(attributes.clone(), tag_name);

                    model.add_verse_to_in_para(model.get_attributes());
                    attributes.clear();
                }
                (TokenType::Eol, _, "book") => {
                    model.add_string_to_in_para(&mut txt);
                    model.end_book();
                    txt.clear();
                    current_state.clear();
                }
                (TokenType::Eol, _, "usfm") => {
                    model.add_root_metadata(&txt.join(""));
                    txt.clear();
                }
                (TokenType::Eol, _, _) => {
                    if txt.is_empty() && in_para == true{
                        let tag_name = "para".to_string();
                        model.push_element(attributes.clone(), tag_name);
                        model.start_new_para(model.get_attributes());
                    } else if in_para == true {
                        model.add_string_to_in_para(&mut txt);
                        model.end_new_para();
                    }
                }
                (TokenType::EndTag, _, "char") => {
                    model.end_add_char_marker(&mut txt);
                    txt.clear();
                }
                // (TokenType::EndTag, "qs*", "char") => {
                //     model.end_add_char_marker(&mut txt);
                //     txt.clear();
                // }
                // (TokenType::EndTag, "w*", "char") => {
                //     model.end_add_char_marker(&mut txt);
                //     txt.clear();
                // }
                // (TokenType::EndTag, "+it*", "char") => {
                //     model.end_add_char_marker(&mut txt);
                //     txt.clear();
                // }
                _ => {}
            }

        } else {
            match (&event.token_type, current_state.as_str(), txt.is_empty()) {
                (TokenType::WordLike, "book", true) => {
                    let tag_name = "book".to_string();
                    attributes.insert("code".to_string(), event.value.clone());
                    model.push_element(attributes.clone(), tag_name);
                    model.start_book(model.get_attributes());
                    attributes.clear();
                },
                (TokenType::WordLike, "para", true) => {
                    let tag_name = "para".to_string();
                    model.push_element(attributes.clone(), tag_name);
                    model.start_new_para(model.get_attributes());
                    txt.push(event.value.as_str().to_string());
                },
                (TokenType::WordLike, "char", true) => {
                    let tag_name = "char".to_string();
                    model.push_element(attributes.clone(), tag_name);
                    model.start_add_char_marker(model.get_attributes());
                },
                _ => { //TODO : gérer les espaces
                    txt.push(event.value.as_str().to_string());
                },
            }

        }


    }
    if in_para == true {
        model.add_string_to_in_para(&mut txt);
        model.end_new_para();
    }


    model.assemble_model()
}



// pub fn print_ast_node_info(node: Node, source_code: &str, depth: usize) {
//     // Print the node type
//     println!("{:indent$}node.kind: {}", "", node.kind(), indent = depth * 4);
//
//     // Iterate through child nodes
//     for child in node.children(&mut node.walk()) {
//         println!("{:indent$}child.kind: {}", "", child.kind(), indent = (depth + 1) * 4);
//
//         // Recursively explore child nodes
//         if child.child_count() > 0 {
//             print_ast_node_info(child, source_code, depth + 1);
//         } else {
//             // For leaf nodes (no children), print the text content
//             if child.is_named() {
//                 let text = child.utf8_text(source_code.as_bytes()).unwrap();
//                 println!("{:indent$}text = {}", "", text, indent = (depth + 2) * 4);
//             }
//         }
//     }
// }
//
//
// fn read_content<T: AosjModel>(model: &mut T, node: Node, source_code: &str ) {
//
//     let mut txt: Vec<String> = Vec::new();
//     // let mut tag_name:String = String::new();
//     if node.kind() == "text" {
//         // println!("{}", node.utf8_text(source_code.as_bytes()).unwrap().to_string());
//         txt.push(node.utf8_text(source_code.as_bytes()).unwrap().to_string());
//     } else {
//         for child in node.children(&mut node.walk()) {
//             read_content(model, child, source_code);
//         }
//     }
//
//     model.add_string_to_in_para(&mut txt);
// }
//
//
//
// fn get_node_attributes(node: Node, source_code: &str) -> BTreeMap<String, String> {
//     let mut attributes = BTreeMap::new();
//
//
//     match node.kind() {
//         "id" => {
//             attributes.insert("marker".to_string(), "id".to_string());
//             if let Some(bookcode) = node.child_by_field_name("bookcode") {
//                 attributes.insert("code".to_string(), bookcode.utf8_text(source_code.as_bytes()).unwrap().to_string());
//             }
//         }
//         "ide" => {
//             attributes.insert("marker".to_string(), "ide".to_string());
//         }
//         "hBlock" => {
//             attributes.insert("marker".to_string(), "h".to_string());
//         }
//         _ => {
//             for child in node.children(&mut node.walk()) {
//                 get_node_attributes(child, source_code);
//             }
//         }
//     }
//
//
//     // for child in node.children(&mut node.walk()) {
//     //     if child.kind() == "attribute" {
//     //         let key = child.child_by_field_name("key").unwrap().utf8_text(source_code.as_bytes()).unwrap().to_string();
//     //         let value = child.child_by_field_name("value").unwrap().utf8_text(source_code.as_bytes()).unwrap().to_string();
//     //         attributes.insert(key, value);
//     //     }
//     // }
//     // println!("attributes : {:#?}", attributes);
//     attributes
// }
//
//
// pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) -> String {
//
//     let mut file = File::open(input_file_path).unwrap();
//     let mut content = String::new();
//     file.read_to_string(&mut content).unwrap();
//
//     let binding = USFMParser(content.as_str());
//     let root_node = binding.root_node();
//     let mut model = T::new();
//
//     let mut attributes: BTreeMap<String, String>= BTreeMap::new();
//
//
//     print_ast_node_info(root_node, content.as_str(), 0);
//
//
//     let mut root_node_binding = root_node.walk();
//     // println!("{}", root_node.to_sexp());
//     let tree = root_node.children(&mut root_node_binding);
//     // let node = tree;
//     for node in tree {
//         // println!("{:?}", node.child(0));
//         //  for child in node.children(&mut node.walk()) {
//         //      println!("{:?}", child.child(2));
//         //
//         //      for mini in child.children(&mut child.walk()) {
//         //          // println!(" mini.walk().node() =={:?}", mini);
//         //      }
//         // }
//
//         // let mut tag_name:String = String::new();
//         // if node.kind() == "usfm" {
//         //
//         // } else if node.kind() == "book" {
//         //     tag_name = node.kind().to_string();
//         // } else if node.kind() == "chapter" {
//         //     tag_name = node.kind().to_string();
//         // } else {
//         //     tag_name = "para".to_string();
//         // }
//
//         let tag_name = node.kind().to_string();
//         // println!("tag_name : {}", tag_name);
//
//         attributes = get_node_attributes(node, content.as_str());
//         model.push_element(attributes.clone(), tag_name.clone());
//
//         // println!("{}", model.get_attributes());
//
//         // if node.kind() == "usfm" {
//         //     model.add_root_metadata(&"3.0".to_string());
//         // } else if node.kind() == "book" {
//         //     model.start_book(model.get_attributes());
//         //
//         //     for child in node.children(&mut node.walk()) {
//         //         read_content(&mut model, child, content.as_str());
//         //     }
//         //
//         //     model.end_book();
//         // } else if node.kind() == "chapter" {
//         //     model.add_chapter(model.get_attributes());
//         // } else {
//         //     model.start_new_para(model.get_attributes());
//         //
//         //     for child in node.children(&mut node.walk()) {
//         //         // println!("{}", content.as_str());
//         //         read_content(&mut model, child, content.as_str());
//         //     }
//         //
//         //     model.end_new_para();
//         // }
//
//
//         match tag_name.as_str() {
//             "usfm" => model.add_root_metadata(&"3.0".to_string()),
//             "book" => {
//                 model.start_book(model.get_attributes());
//                 for child in node.children(&mut node.walk()) {
//                     read_content(&mut model, child, content.as_str());
//                 }
//                 model.end_book();
//             }
//             "chapter" => model.add_chapter(model.get_attributes()),
//             _ => {
//                 model.start_new_para(model.get_attributes());
//                 for child in node.children(&mut node.walk()) {
//                     read_content(&mut model, child, content.as_str());
//                 }
//                 model.end_new_para();
//             }
//         }
//
//     }
//
//
//     // model.assemble_model()
//     root_node.to_sexp()
// }
//
