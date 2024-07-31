use tempfile::NamedTempFile;
use std::io::Write;
use deserialize_usx::deserialize_from_file_usx;


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

    let result = deserialize_from_file_usx::<AosjStringModel>(file_path);

    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse result JSON");

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

    deserialize_from_file_usx::<AosjStringModel>(file_path);
}
