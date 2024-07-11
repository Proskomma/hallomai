use std::fs::File;
#[allow(dead_code)]

use tree_sitter_usfm_test;
use tree_sitter;
use tree_sitter::Node;

use crate::model_traits::AosjModel;


pub fn print_ast_node_info(node: Node, source_code: &str, depth: usize) {
    // Print the node type
    println!("{:indent$}Node type: {}", "", node.kind(), indent = depth * 2);

    // Iterate through child nodes
    for child in node.children(&mut node.walk()) {
        println!("{:indent$}- Child node type: {}", "", child.kind(), indent = (depth + 1) * 2);

        // Recursively explore child nodes
        if child.child_count() > 0 {
            print_ast_node_info(child, source_code, depth + 1);
        } else {
            // For leaf nodes (no children), print the text content
            if child.is_named() {
                let text = child.utf8_text(source_code.as_bytes()).unwrap();
                println!("{:indent$}- Text content: {}", "", text, indent = (depth + 2) * 2);
            }
        }
    }
}

pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) {

    let mut file = File::open(input_file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let wrapped_content = format!(r#"{}"#, content);

    let mut model = T::new();


}
