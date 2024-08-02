use std::fs;
use std::path::Path;
use deserialize_usfm::deserialize_from_file_path_usfm;

#[test]
fn it_deserialize_usfm() {

    let file_path = "tests/datas/usfm/good/cl.usfm";
    let result = deserialize_from_file_path_usfm::<AosjStringModel>(file_path);
    let result_json: Value = serde_json::from_str(&result).unwrap();

    assert_eq!(result_json.get("version").unwrap(), "3.0");

    assert_eq!(result_json.get("content").unwrap()[0].get("code").unwrap(), "PSA");
    assert_eq!(result_json.get("content").unwrap()[2].get("marker").unwrap(), "sts");
    assert_eq!(result_json.get("content").unwrap()[11].get("content").unwrap()[2].get("default").unwrap(), "Man");
    assert_eq!(result_json.get("content").unwrap()[11].get("content").unwrap()[4].get("content").unwrap()[0].get("content").unwrap()[0], "does not");
    assert_eq!(result_json.get("content").unwrap()[13].get("content").unwrap()[6].get("x-occurrence").unwrap(), "1");

}

#[test]
fn test_deserialize_multiple_usfms() {
    let path = Path::new("tests/datas/usfm/good/");

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("usfm") {
            deserialize_from_file_path_usfm::<AosjStringModel>(path.to_str().unwrap());
        }
    }
}

#[test]
#[should_panic]
fn fail_parse_usfm() {
    let file_path = "tests/datas/usfm/bad/cl_bad.usfm";
    deserialize_from_file_path_usfm::<AosjStringModel>(file_path);
}
