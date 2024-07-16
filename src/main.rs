//! This crate is a tool for reading and processing USX files.
//! It uses custom modules and the SAX approach to parse the files
//! and convert the data into structured models.
//!
//! ### Organisation
//!
//! The crate is organized into several modules:
//!
//! - `element`: Defines the `Element` struct for representing XML elements.
//! - `aosj_string_model`: Contains the main model `AosjStringModel` for processing USX documents.
//! - `model_traits`: Defines the `AosjModel` trait for model operations.
//! - `deserialize_usx`: Provides functions for reading and parsing USX files using SAX.
//!

#![allow(dead_code)]
mod model_traits;
mod aosj_string;

use aosj_string::aosj_string_model::AosjStringModel;

mod deserialize_usx;
mod deserialize_usj;
mod deserialize_usfm;
mod reg_ex_tests;

extern crate rust_usfm_parser;

// use regex::Regex;
// use std::collections::HashMap;
// use regex::{Captures, Regex};
// use tree_sitter::Node;
// use rust_usfm_parser::USFMParser;
// use crate::aosj_string::aosj_string_model::AosjStringModel;


/// This function initializes the deserialization process for a USX file and
/// processes it using the `AosjStringModel`.
fn main() {

    // let input_file_path = "./assets/usx/milestone_attributes.usx";
    // let input_file_path = "./assets/usj/small.json";
    let input_file_path = "./assets/usfm/cl.usfm";

    if input_file_path.ends_with(".usx") {
        // let a = deserialize_usx::deserialize_from_file::<AosjStringModel>(input_file_path);
        // println!("{}",a);
    } else if input_file_path.ends_with(".json") {
        // let b = deserialize_usj::deserialize_from_file::<AosjStringModel>(input_file_path);
        // println!("{}",b);
    } else if input_file_path.ends_with(".usfm") {
        // let c = deserialize_usfm::deserialize_from_file::<AosjStringModel>(input_file_path);


        let c = deserialize_usfm::deserialize_from_file::<AosjStringModel>(input_file_path);
        println!("{}", c);

    }

    // reg_ex_tests::extract_usfm_attributes(input_file_path);




         // let usfm_code = r#"\id MAT some other info of file
// \c 1
// \p
// \v 11 Jesus stood before the Roman governor, who questioned him.
// \qt-s |who="Pilate"\* "Are
// you the king of the Jews?"\qt-e\* he asked.
// \p \qt-s |who="Jesus"\*"So you say,"\qt-e\* answered Jesus.
// \v 12 But he said nothing in response to the accusations of the chief priests and elders.
// \p
// \v 13 So Pilate said to him, \qt-s |who="Pilate"\*"Don't you hear all these things they
// accuse you of?"\qt-e\*
// \p
// \v 14 But Jesus refused to answer a single word, with the result that the Governor was greatly
// surprised."#;
//
//
//     deserialize_usfm::print_ast_node_info(USFMParser(usfm_code).root_node(), usfm_code, 0);
}

