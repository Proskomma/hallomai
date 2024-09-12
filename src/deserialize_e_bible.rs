use std::collections::BTreeMap;
use regex::Regex;
use crate::utils_usfm;
use crate::model_traits::AosjModel;


/// # Reads the eBible file and reconstructs it into an AosjModel.
///
/// This function processes an eBible file, with all the structs defined.
/// It reconstructs the file into a model that implements the `AosjModel`
/// trait.


pub fn deserialize_from_file_e_bible<T: AosjModel>(content: String) -> String {

    let mut model = T::new();


    model.assemble_model()
}

