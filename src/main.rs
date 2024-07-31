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
mod utils_usfm;


// include!("../tests/code/test_deserialize_usj.rs");
// include!("../tests/code/test_deserialize_usx.rs");
// include!("../tests/code/test_deserialize_usfm.rs");

// extern crate rust_usfm_parser;

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
        let a = deserialize_usx::deserialize_from_file_path_usx::<AosjStringModel>(input_file_path);
        println!("{}",a);
    } else if input_file_path.ends_with(".json") {
        let b = deserialize_usj::deserialize_from_file_path_usj::<AosjStringModel>(input_file_path);
        println!("{}",b);
    } else if input_file_path.ends_with(".usfm") {
        let c = deserialize_usfm::deserialize_from_file_path_usfm::<AosjStringModel>(input_file_path);
        println!("{}", c);

    }
}

