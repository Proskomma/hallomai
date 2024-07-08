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


/// This function initializes the deserialization process for a USX file and
/// processes it using the `AosjStringModel`.
fn main() {

    let input_file_path = "./assets/web_psa150.usx";

    if input_file_path.ends_with(".usx") {
        deserialize_usx::deserialize_from_file::<AosjStringModel>(input_file_path);
    } else if input_file_path.ends_with(".json") {
        deserialize_usj::deserialize_from_file::<AosjStringModel>(input_file_path)
    }
}
