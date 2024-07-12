use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
#[allow(dead_code)]

use rust_usfm_parser;
use rust_usfm_parser::USFMParser;
use tree_sitter;
use tree_sitter::Node;

use crate::model_traits::AosjModel;


pub fn print_ast_node_info(node: Node, source_code: &str, depth: usize) {
    // Print the node type
    println!("{:indent$}node.kind: {}", "", node.kind(), indent = depth * 2);

    // Iterate through child nodes
    for child in node.children(&mut node.walk()) {
        println!("{:indent$}- child.kind: {}", "", child.kind(), indent = (depth + 1) * 2);

        // Recursively explore child nodes
        if child.child_count() > 0 {
            print_ast_node_info(child, source_code, depth + 1);
        } else {
            // For leaf nodes (no children), print the text content
            if child.is_named() {
                let text = child.utf8_text(source_code.as_bytes()).unwrap();
                println!("{:indent$}- text = child.utf8_text: {}", "", text, indent = (depth + 2) * 2);
            }
        }
    }
}


fn read_content<T: AosjModel>(model: &mut T, node: Node, source_code: &str ) {

    let mut txt: Vec<String> = Vec::new();
    // let mut tag_name:String = String::new();

    if node.kind() == "text" {
        txt.push(node.utf8_text(source_code.as_bytes()).unwrap().to_string());
    }
    model.add_string_to_in_para(&mut txt);
}

pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) -> String {

    let mut file = File::open(input_file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let binding = USFMParser(content.as_str());
    let root_node = binding.root_node();
    let mut model = T::new();

    // let mut attributes: BTreeMap<String, String>= BTreeMap::new();


    // print_ast_node_info(root_node, content.as_str(), 0);
    let mut root_node_binding = root_node.walk();
    // println!("{}", root_node.to_sexp());
    let tree = root_node.children(&mut root_node_binding);
    // let node = tree;
    for node in tree {
        // println!("{:?}", node.child(0));
        //  for child in node.children(&mut node.walk()) {
        //      println!("{:?}", child.child(2));
        //
        //      for mini in child.children(&mut child.walk()) {
        //          // println!(" mini.walk().node() =={:?}", mini);
        //      }
        // }

        let mut tag_name:String = String::new();
        if node.kind() == "usfm" {

        } else if node.kind() == "book" {
            tag_name = node.kind().to_string();
        } else if node.kind() == "chapter" {
            tag_name = node.kind().to_string();
        } else {
            tag_name = "para".to_string();
        }
        // println!("tag_name : {}", tag_name);
        // model.push_element(attributes.clone(), tag_name);

        if node.kind() == "usfm" {
            model.add_root_metadata(&"3.0".to_string());
        } else if node.kind() == "book" {
            model.start_book(model.get_attributes());

            for child in node.children(&mut node.walk()) {
                read_content(&mut model, child, content.as_str());
            }

            model.end_book();
        } else if node.kind() == "chapter" {
            model.add_chapter(model.get_attributes());
        } else {
            model.start_new_para(model.get_attributes());

            for child in node.children(&mut node.walk()) {
                read_content(&mut model, child, content.as_str());
            }

            model.end_new_para();
        }
    }


    model.assemble_model()
    // root_node.to_sexp()
}

