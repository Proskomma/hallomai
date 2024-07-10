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

use crate::aosj_string::aosj_string_model::AosjStringModel;

mod deserialize_usx;
mod deserialize_usj;
mod parser;


/// This function initializes the deserialization process for a USX file and
/// processes it using the `AosjStringModel`.
fn main() {

    // let input_file_path = "./assets/small.json";
    let input_file_path = "./assets/milestone_attributes.usx";


    if input_file_path.ends_with(".usx") {
        // println!("{}",deserialize_usx::deserialize_from_file::<AosjStringModel>(input_file_path));
        // let a = deserialize_usx::deserialize_from_file::<AosjStringModel>(input_file_path);
    } else if input_file_path.ends_with(".json") {
        // println!("{}",deserialize_usj::deserialize_from_file::<AosjStringModel>(input_file_path));
        // let a = deserialize_usj::deserialize_from_file::<AosjStringModel>(input_file_path);
    }

    let usfm_code = r"\p This is a paragraph \v 1 This is a verse";
    unsafe { parser::parse_usfm(usfm_code); }



    // let language = "usfm";
    // let package = "proskomma_2".to_string();
    // let source_directory = format!("{}/src", package);
    // let source_file = format!("{}/parser.c", source_directory);
    //
    // println!("cargo:rerun-if-changed={}", "parser.c"); // <1>
    //
    // cc::Build::new()
    //     .file("parser.c")
    //     .include("src")
    //     .compile(&package); // <2>
}

