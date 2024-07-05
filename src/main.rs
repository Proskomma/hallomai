#![allow(dead_code)]
mod structs_model;
mod model_traits;
mod aosj_string;

use crate::aosj_string::aosj_string_model::AosjStringModel;

mod deserialize_usx;



fn main() {

    let input_file_path = "./assets/web_psa150.usx";

    if input_file_path.ends_with(".usx") {
        deserialize_usx::deserialize_from_file::<AosjStringModel>(input_file_path);
    }
}
