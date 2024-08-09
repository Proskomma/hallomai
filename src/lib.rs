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

use serde_json::to_value;
use wasm_bindgen::prelude::*;
mod model_traits;
mod aosj_string;
mod structs_model;
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

use structopt::StructOpt;
use crate::deserialize_usfm::deserialize_from_file_usfm;
use crate::deserialize_usj::{deserialize_from_file_str_usj, deserialize_from_file_usj};
use crate::deserialize_usx::deserialize_from_file_usx;
use crate::model_traits::AosjModel;

include!("../tests/code/test_deserialize_usj.rs");
include!("../tests/code/test_deserialize_usx.rs");
include!("../tests/code/test_deserialize_usfm.rs");


/// Transforms a USFM file into a different format (`.usfm`, `.usx`, or `.json`).
///
/// # Parameters
/// - `input_file`: A `String` representing the content of the file. The input file must be in `.usfm` format.
/// - `output_file_format`: A `String` specifying the desired output file format. Only `usfm`, `usx`, and `json` (i.e. usj) formats are supported.
///
/// # Returns
/// A `String` of the desired output format. If the output format is unsupported,
/// it returns an error message indicating that the output file format is not supported.
///
/// # Panics
/// The function may panic if there are issues during file serialization or deserialization processes.
///
/// # Example
/// ```
/// let result = transform_file_to("{YOUR FILE CONTENT}", "usx");
/// console.log(result);
/// ```
///
/// # Note
/// This function is designed to handle `.usfm` files as input and supports the following output formats:
/// - `.usfm`: Transforms the input file into another `.usfm` format.
/// - `.usx`: Converts the input file into `.usx` format.
/// - `.json`: Converts the input file into `.json` format.
///
/// If an unsupported output file format is provided, the function will return an error message.
#[wasm_bindgen]
pub fn transform(input_file_content: String, input_file_format: String, output_file_format: String) -> String {
    let model = match input_file_format.as_str() {
        "usx" => deserialize_from_file_usx::<AosjStringModel>(input_file_content),
        "usfm" => deserialize_from_file_usfm::<AosjStringModel>(input_file_content),
        "json" | "usj" => deserialize_from_file_usj::<AosjStringModel>(serde_json::from_str(&input_file_content).unwrap()),
        _ => return "Unsupported input file format. Only 'usfm', 'usx', and 'json' are supported.".to_string(),
    };

    match output_file_format.as_str() {
        "json" | "usj" => serialize_to_usj::serialize_to_usj(model).to_string(),
        "usfm" => {
            let usj = serialize_to_usj::serialize_to_usj(model);
            serialize_to_usfm::serialize_to_usfm(usj)
        }
        "usx" => {
            let usj = serialize_to_usj::serialize_to_usj(model);
            serialize_to_usx::serialize_to_usx(usj)
        }
        _ => "Unsupported output file format. Only 'usfm', 'usx', and 'json' are supported.".to_string(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const USFM_CONTENT: &str = r#"\\id PSA unfoldingWord Literal Text
\\usfm 3.0
\\ide UTF-8
\\sts 2
\\h Psalms
\\toc1 The Book of Psalms
\\toc2 Psalms
\\toc3 Psa
\\mt Psalms
\\c 1
\\s Here comes a psalm
\\s2 See also all the other psalms
\\q
\\v 1 Blessed is the \\w man|Man\\w* who \\bd \\+it does not\\+it* walk\\bd* in the advice of the wicked,
\\q or stand in the pathway with sinners,
\\q or sit in the assembly of mockers.\\qs Selah\\qs* Amen
\\ts\\*
\\v 2 Beginning \\zaln-s |x-strong="G5043" x-lemma="τέκνον" x-morph="Gr,N,,,,,NNP," x-occurrence="1" x-occurrences="1" x-content="τέκνα"\\*\\w milestone |x-occurrence="1" x-occurrences="1"\\w*\\zaln-e\\*
"#;

    #[test]
    fn test_transform_usfm_to_json() {
        let output = transform(USFM_CONTENT.to_string(), "usfm".to_string(), "json".to_string());
        assert!(output.contains("\"usfm\":"));
        assert!(output.contains("\"toc1\": \"The Book of Psalms\""));
    }

    #[test]
    fn test_transform_usfm_to_usx() {
        let output = transform(USFM_CONTENT.to_string(), "usfm".to_string(), "usx".to_string());
        assert!(output.contains("<usx>"));
        assert!(output.contains("<toc1>The Book of Psalms</toc1>"));
    }

    #[test]
    fn test_transform_usfm_to_usfm() {
        let output = transform(USFM_CONTENT.to_string(), "usfm".to_string(), "usfm".to_string());
        assert_eq!(output, USFM_CONTENT);
    }

    #[test]
    fn test_invalid_input_format() {
        let output = transform(USFM_CONTENT.to_string(), "invalid_format".to_string(), "json".to_string());
        assert_eq!(output, "Unsupported input file format. Only 'usfm', 'usx', and 'json' are supported.".to_string());
    }

    #[test]
    fn test_invalid_output_format() {
        let output = transform(USFM_CONTENT.to_string(), "usfm".to_string(), "invalid_format".to_string());
        assert_eq!(output, "Unsupported output file format. Only 'usfm', 'usx', and 'json' are supported.".to_string());
    }
}
