#![allow(dead_code)]

use std::collections::BTreeMap;
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::model_traits::AosjModel;

/// # Reads the USX file and reconstructs it into an AosjModel.
///
/// This function processes a USX file, parsing its
/// content and reconstructing it into a model that implements the `AosjModel`
/// trait. It handles different types of XML events such as start tags, end tags,
/// empty elements, and text nodes.
pub fn deserialize_from_file<T:AosjModel>(input_file_path: &str) -> String {
    let mut reader = Reader::from_file(input_file_path).unwrap();
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut txt = Vec::new();

    let mut model = T::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(el)) => {
                model.add_string_to_in_para(
                    &mut txt
                );
                let mut attributes: BTreeMap<String, String> = BTreeMap::new();

                for att in el.attributes() {
                    attributes.insert(
                        String::from_utf8(att.clone().unwrap().key.local_name().as_ref().to_vec()).unwrap(),
                        String::from_utf8(att.clone().unwrap().value.as_ref().to_vec()).unwrap()
                    );
                }

                let tag_name = String::from_utf8(el.name().as_ref().to_vec()).unwrap();

                model.push_element(attributes, tag_name.clone());

                let current_parent = model.parent_els().clone();

                if tag_name == "usx" {
                    model.add_root_metadata(
                        current_parent.last().unwrap().attributes.get("version").unwrap(),
                    );
                } else if tag_name == "para" {
                    model.start_new_para(
                        model.get_attributes()
                    );

                } else if tag_name == "book" {
                    model.start_book(
                        model.get_attributes()
                    )
                } else if tag_name == "char" {
                    model.start_add_char_marker(
                        model.get_attributes()
                    )
                } else if tag_name == "note" {
                    model.start_add_note(
                        model.get_attributes()
                    )
                }
            }

            Ok(Event::Empty(el)) => {
                model.add_string_to_in_para(&mut txt);
                let mut attributes: BTreeMap<String, String> = BTreeMap::new();
                for att in el.attributes() {
                    attributes.insert(
                        String::from_utf8(att.clone().unwrap().key.local_name().as_ref().to_vec()).unwrap(),
                        String::from_utf8(att.clone().unwrap().value.as_ref().to_vec()).unwrap()
                    );
                }
                let tag_name = String::from_utf8(el.name().as_ref().to_vec()).unwrap();

                model.push_element(attributes, tag_name.clone());

                if tag_name == "verse" && !model.parent_els().last().unwrap().attributes.contains_key("eid") {
                    model.add_verse_to_in_para(
                        model.get_attributes()
                    );
                } else if tag_name == "chapter" && !model.parent_els().last().unwrap().attributes.contains_key("eid") {
                    model.add_chapter(
                        model.get_attributes()
                    );
                } else if tag_name == "ms" && !model.parent_els().last().unwrap().attributes.contains_key("eid") {

                    model.add_milestone(
                        model.get_attributes()
                    )
                }

                model.parent_els().pop();
            }

            Ok(Event::Text(el)) => {
                if model.parent_els().len()>1 {
                    txt.push(el.unescape().unwrap().into_owned());
                }
                println!("{:#?}", txt);

            }

            Ok(Event::End(..)) => {

                let tag_name = model.parent_els().last().unwrap().tag_name.clone();

                if tag_name == "para" {
                    model.add_string_to_in_para(
                        &mut txt
                    );
                    model.end_new_para()
                } else if tag_name == "book" {
                    model.add_string_to_in_para(
                        &mut txt
                    );
                    model.end_book()
                } else if tag_name == "char" {
                    model.end_add_char_marker(
                        &mut txt
                    )
                } else if tag_name == "note" {
                    model.end_add_note(
                        &mut txt
                    )
                }
                model.parent_els().pop();
            }

            Ok(Event::Eof) => {
                return model.assemble_model();
            }
            Err(err) => {
                panic!("Error reading XML: {}", err);
            }
            _ => {}
        }
        buf.clear();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use serde_json::Value;
    use crate::aosj_string::aosj_string_model::AosjStringModel;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        write!(file, "{}", content).expect("Failed to write to temp file");
        file
    }

    #[test]
    fn it_deserialize_usx() {

        let usx_content = r#"
            <usx version="2.0">
                <book code="PSA" style="id">Psalm</book>
                <chapter number="1" style="c" sid="PSA 1:1"/>
                <para style="p">
                    <verse number="1" style="v" sid="PSA 1:1"/>
                    Praise the <char style="nd">Lord</char>, for he is good.
                    <note caller="*" style="f">
                        <ms style="zaln-s" sid="aln1" x-strong="H5662" x-lemma="עֹבַדְיָה" x-morph="He,Np" x-occurrence="1"
                        x-occurrences="1" x-content="עֹֽבַדְיָ֑ה"/>
                        Footnote content
                        <char style="qs"><char style="fr">Selah.</char></char>
                    </note>
                </para>
                <para style="q">
                    God's love never fails.
                </para>
            </usx>
        "#;

        let temp_file = create_temp_file(usx_content);
        let file_path = temp_file.path().to_str().unwrap();

        let result = deserialize_from_file::<AosjStringModel>(file_path);

        let result_json: Value = serde_json::from_str(&result).expect("Failed to parse result JSON");
        // println!("{:#?}", result_json);

        assert_eq!(result_json.get("version").unwrap().as_str().unwrap(), "2.0");

        assert_eq!(result_json.get("content").unwrap()[0].get("code").unwrap(), "PSA");
        assert_eq!(result_json.get("content").unwrap()[2].get("content").unwrap()[1], "Praise the");
        assert_eq!(result_json.get("content").unwrap()[2].get("content").unwrap()[4].get("content").unwrap()[0].get("x-content").unwrap(), "עֹֽבַדְיָ֑ה");
        assert_eq!(result_json.get("content").unwrap()[2].get("content").unwrap()[4].get("content").unwrap()[2].get("content").unwrap()[0].get("content").unwrap()[0], "Selah.");
        assert_eq!(result_json.get("content").unwrap()[3].get("content").unwrap()[0], "God's love never fails.");

    }

    #[test]
    #[should_panic]
    fn fail_parse_usx() {
        let usx_content = r#"
            <usx version="2.0">
                <book code="PSA" style="id">Psalm</book>
                <chapter number="1" style="c" sid="PSA 1:1"/>
                <para style="p"
                    <verse number="1" style="v" sid="PSA 1:1"/>
                    Praise the <char style="nd">Lord</char>, for he is good.
                </para>
            </usx>
        "#;

        let temp_file = create_temp_file(usx_content);
        let file_path = temp_file.path().to_str().unwrap();

        let result = deserialize_from_file::<AosjStringModel>(file_path);
    }

}
