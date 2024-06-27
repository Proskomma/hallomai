use crate::deserialize_usj::USJ;
// use std::convert::Infallible;
// use crate::deserialize_usj::USJ;
// use crate::deserialize_usx::USX;
use crate::model::{DataType, DataTypeTraits};

mod model;
mod deserialize_usj;
mod deserialize_usx;

fn main() {

    let the_json = "{ \
        \"type\": \"USJ\", \
        \"version\": \"une version\", \
        \"content\": \"pas content\" \
    }";

    let the_xml = "<usx version=\"3.0\">mon pas content</usx>";
    // let usj: DataType = serde_json::from_str(the_json).unwrap();
    let usj: DataType = serde_json::from_str(the_json).unwrap();

    // let usx: DataType = serde_xml_rs::from_str(the_xml).unwrap();

    println!("{:#?}",usj);
    // usx.print_version();
}
