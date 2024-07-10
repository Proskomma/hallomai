extern crate tree_sitter;
extern crate tree_sitter_usfm3;

use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_usfm() -> Language; }

pub unsafe fn parse_usfm(source_code: &str) {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_usfm()).expect("Error loading USFM grammar");

    let tree = parser.parse(source_code, None).unwrap();
    println!("{:?}", tree.root_node().to_sexp());
}