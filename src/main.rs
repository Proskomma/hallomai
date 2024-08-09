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
mod structs_model;

use std::fs::File;
use std::io::Read;
use aosj_string::aosj_string_model::AosjStringModel;

mod deserialize_usx;
mod deserialize_usj;
mod deserialize_usfm;
mod reg_ex_tests;
mod utils_usfm;
mod aosj_enum_model;
mod serialize_to_usj;
mod serialize_to_usx;
mod serialize_to_usfm;
use serde_json::to_value;
use structopt::StructOpt;
use crate::deserialize_usfm::deserialize_from_file_usfm;
use crate::deserialize_usj::deserialize_from_file_usj;
use crate::deserialize_usx::deserialize_from_file_usx;


include!("../tests/code/test_deserialize_usj.rs");
include!("../tests/code/test_deserialize_usx.rs");
include!("../tests/code/test_deserialize_usfm.rs");

// extern crate rust_usfm_parser;

// use regex::Regex;
// use std::collections::HashMap;
// use regex::{Captures, Regex};
// use tree_sitter::Node;
// use rust_usfm_parser::USFMParser;
// use crate::aosj_string::aosj_string_model::AosjStringModel;

#[derive(StructOpt, Debug)]
#[structopt(name = "usx_tool")]
struct Opt {
    /// Input file path
    #[structopt(short, long)]
    input: String,

    /// Output file path
    #[structopt(short, long)]
    output: String,
}


/// This function initializes the deserialization process for a USX file and
/// processes it using the `AosjStringModel`.
fn main() {

    let opt = Opt::from_args();

    let mut input_file_path = opt.input;
    let output = opt.output;

    let mut file = File::open(input_file_path.clone()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // use std::time::Instant;
    // let now = Instant::now();

    if input_file_path.ends_with("usx") {
        let model = deserialize_from_file_usx::<AosjStringModel>(content);
        // println!("{}", model);
        if output == "json" || output == "usj" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            println!("{}", usj);
        } else if output == "usfm" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            let usfm = serialize_to_usfm::serialize_to_usfm(usj);
            println!("{}", usfm);
        } else if output == "usx" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            let usx = serialize_to_usx::serialize_to_usx(usj);
            println!("{}", usx);
        } else {
            eprintln!("Unsupported output file format for .usx input.");
        }
    } else if input_file_path.ends_with("json") {
        let model = deserialize_from_file_usj::<AosjStringModel>(serde_json::from_str(content.as_str()).unwrap());
        if output == "usx" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            let usx = serialize_to_usx::serialize_to_usx(usj);
            println!("{}", usx);
        } else if output == "usfm" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            let usfm = serialize_to_usfm::serialize_to_usfm(usj);
            println!("{}", usfm);
        } else if output == "json" || output == "usj" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            println!("{}", usj);
        } else {
            eprintln!("Unsupported output file format for .usj input.");        }
    } else if input_file_path.ends_with("usfm") {
        let model = deserialize_from_file_usfm::<AosjStringModel>(content);
        if output == "json" || output == "usj" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            println!("{}", usj);
        } else if output == "usx" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            let usx = serialize_to_usx::serialize_to_usx(usj);
            println!("{}", usx);
        } else if output == "usfm" {
            let usj = serialize_to_usj::serialize_to_usj(model);
            let usfm = serialize_to_usfm::serialize_to_usfm(usj);
            println!("{}", usfm);
        } else {
            eprintln!("Unsupported output file format for .usfm input.");
        }
    } else {
        eprintln!("Unsupported input file format.");
    }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);


    // pour run le code : cargo run -- --input chemin_vers_le_fichier_d_entrée --output extension_fichier_de_sortie>nom_du_fichier_de_sortie


    // // let input_file_path = "PSA.usx";
    // // let input_file_path = "./assets/usj/small.json";
    // let input_file_path = "./assets/usfm/65-3JN.usfm";
    //
    // if input_file_path.ends_with(".usx") {
    //     let a = deserialize_from_file_path_usx::<AosjStringModel>(input_file_path);
    //     // println!("{}", a);
    //     let d = serialize_to_usj::serialize_to_usj(a, "output_usx_to_usj.json");
    //     serialize_to_usx::serialize_to_usx(d, "output_usj_to_usx.usx");
    // } else if input_file_path.ends_with(".json") {
    //     let b = deserialize_from_file_path_usj::<AosjStringModel>(input_file_path);
    //     serialize_to_usj::serialize_to_usj(b, "output_usj.json");
    // } else if input_file_path.ends_with(".usfm") {
    //     let c = deserialize_from_file_path_usfm::<AosjStringModel>(input_file_path);
    //     // println!("{}", c);
    //     let f = serialize_to_usj::serialize_to_usj(c, "output_usfm_to_usj.json");
    //     serialize_to_usfm::serialize_to_usfm(f.clone(), "output_usj_to_usfm.usfm");
    //     // serialize_to_usx::serialize_to_usx(f, "output_usfm_to_usx.usx")
    // }
}

