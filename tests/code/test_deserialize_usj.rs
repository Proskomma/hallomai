
use proskomma_2::deserialize_usj::deserialize_from_file;
use proskomma_2::aosj_string::aosj_string_model::AosjStringModel;

#[test]
fn it_deserialize_usj() {
    let file_path = "./tests/datas/usj/small.json";
    let result = deserialize_from_file::<AosjStringModel>(file_path);
    let result_json: Value = serde_json::from_str(&result).unwrap();
    assert_eq!(result_json.get("version").unwrap(), "0.2.1");
    assert!(result_json["content"].to_string().contains("41MATGNT92.SFM"));
}
