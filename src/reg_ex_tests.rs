use std::fs::File;
use std::io::Read;
use regex::Regex;
// use crate::model_traits::AosjModel;

use std::collections::BTreeMap;


pub fn use_reg_ex () {
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();

    let haystack = r#"regex = 0.2.5"#;

    assert!(semver.is_match(haystack));

    let captures = semver.captures(haystack)
        .ok_or("semver regex should have matched").unwrap();
    println!("{:#?}",captures);

    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    let haystack_2 = "In the beginning, there was 1.0.0. \
                            For a while, we used 1.0.1-beta, \
                            but in the end, we settled on 1.2.4.";

    let matches: Vec<&str> = semver.find_iter(haystack_2).map(|match_| match_.as_str()).collect();
    println!("{:#?}",matches);
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

}



pub fn extract_usfm_attributes(input_file_path: &str) -> BTreeMap<String, String> {
    let mut file = File::open(input_file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();


    let mut attributes = BTreeMap::new();

    let re_id = Regex::new(r"\\id (\w+)").unwrap();
    let re_ide = Regex::new(r"\\ide (\w+)").unwrap();
    let re_h = Regex::new(r"\\h (.+)").unwrap();
    let re_toc = Regex::new(r"\\toc(\d) (.+)").unwrap();

    println!("re_id : {}, re_ide : {}, re_h : {}, re_toc : {}", re_id, re_ide, re_h, re_toc);

    if let Some(caps) = re_id.captures(&content) {
        attributes.insert("marker".to_string(), "id".to_string());
        attributes.insert("code".to_string(), caps[1].to_string());
    }
    if let Some(caps) = re_ide.captures(&content) {
        attributes.insert("marker".to_string(), "ide".to_string());
        attributes.insert("encoding".to_string(), caps[1].to_string());
    }
    if let Some(caps) = re_h.captures(&content) {
        attributes.insert("marker".to_string(), "h".to_string());
        attributes.insert("title".to_string(), caps[1].to_string());
    }
    if let Some(caps) = re_toc.captures(&content) {
        attributes.insert(format!("toc{}", &caps[1]), caps[2].to_string());
    }

    println!("attributes : {:#?}", attributes);
    attributes
}



