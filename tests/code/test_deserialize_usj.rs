use serde_json::Value;

use deserialize_usj::deserialize_from_file_path_usj;

#[test]
fn it_deserialize_usj() {
    let file_path = "tests/datas/usj/good/small.json";
    let result = deserialize_from_file_path_usj::<AosjStringModel>(file_path);
    let result_json: Value = serde_json::from_str(&result).unwrap();


    assert_eq!(result_json.get("version").unwrap(), "0.2.1");

    assert_eq!(result_json.get("content").unwrap()[0].get("code").unwrap(), "MAT");
    assert_eq!(result_json.get("content").unwrap()[10].get("content").unwrap()[1], "Praise the ");
    assert_eq!(result_json.get("content").unwrap()[11].get("content").unwrap()[0], "God's love never fails ");
    assert_eq!(result_json.get("content").unwrap()[11].get("content").unwrap()[1].get("content").unwrap()[0], "Selah");
}

#[test]
fn test_deserialize_multiple_usjs() {
    let path = Path::new("tests/datas/usj/good/");

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("usj") {
            deserialize_from_file_path_usfm::<AosjStringModel>(path.to_str().unwrap());
        }
    }
}
#[test]
#[should_panic]
fn fail_parse_json() {
    let file_path = "tests/datas/usj/bad/bad_json.json";
    deserialize_from_file_path_usj::<AosjStringModel>(file_path);
}