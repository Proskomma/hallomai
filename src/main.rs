use crate::deserialize_usj::Root;

mod model;
mod deserialize_usj;

fn main() {

    let the_json = "{ \
        \"type\": \"un pov type\", \
        \"version\": \"une version\", \
        \"content\": \"pas content\" \
    }";
    let usj: model::Root = serde_json::from_str(the_json).unwrap();
    usj.print_version();
}
