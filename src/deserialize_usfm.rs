use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
use regex::Regex;
use crate::utils_usfm;
use crate::model_traits::AosjModel;


/// # Reads the USFM file and reconstructs it into an AosjModel.
///
/// This function processes a USFM file, with all the structs defined.
/// It reconstructs the file into a model that implements the `AosjModel`
/// trait.


#[derive(Debug, PartialEq, Clone)]
enum Token {
    Chapter(Chapter),
    Verses(Verses),
    Attribute(Attribute),
    PubChapter(PubChapter),
    Milestone(Milestone),
    Tag(Tag),
    Printable(Printable),
    Bad(Printable),
    Break(Printable),
}

#[derive(Debug, PartialEq, Clone)]
struct Printable {
    subclass: String,
    print_value: String,
}

fn make_printable(subclass: &str, matched_bits: Vec<&str>) -> Printable {
    let mut print_value = matched_bits[0].replace("~", "\u{00a0}");
    let values: Vec<String> = vec!["\"".to_string(), "\\".to_string()];
    if values.contains(&print_value) {
        let value = format!("\\{}", &print_value);
        print_value = print_value.replace(&print_value, value.as_str());
    }
    Printable {
        subclass: subclass.to_string(),
        print_value,
    }
}

fn make_break(subclass: &str, matched_bits: Vec<&str>) -> Printable {
    Printable {
        subclass: subclass.to_string(),
        print_value: matched_bits[0].replace("~", "\u{00a0}"),
    }
}

fn make_bad(subclass: &str, matched_bits: Vec<&str>) -> Printable {
    Printable {
        subclass: subclass.to_string(),
        print_value: matched_bits[0].replace("~", "\u{00a0}"),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Chapter {
    subclass: String,
    number_string: String,
    number: i32,
}

fn make_chapter(subclass: &str, matched_bits: Vec<&str>) -> Chapter {
    let number_string = matched_bits[2].to_string();

    Chapter {
        subclass: subclass.to_string(),
        number_string: number_string.clone(),
        number: number_string.parse::<i32>().unwrap(),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Verses {
    subclass: String,
    number_string: String,
}

fn make_verses(subclass: &str, matched_bits: Vec<&str>) -> Verses {
    let number_string = matched_bits[2].to_string();

    Verses {
        subclass: subclass.to_string(),
        number_string: number_string.clone(),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Attribute {
    subclass: String,
    key: String,
    value_string: String,
    values: Vec<String>,
}

fn make_attribute(subclass: &str, matched_bits: Vec<&str>) -> Attribute {
    let (key, value_string): (String, String);
    if subclass == "defaultAttribute" {
        (key, value_string) = ("default".to_string(), matched_bits[2].trim().replace("/", "+"));
    } else {
        (key, value_string) = (matched_bits[2].to_string(), matched_bits[3].trim().replace("/", "+"));
    }
    let values: Vec<String> = value_string.split(',').map(|v| v.trim().to_string()).collect();
    Attribute {
        subclass: subclass.to_string(),
        key,
        value_string,
        values,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PubChapter {
    subclass: String,
    number_string: String,
}

fn make_pub_chapter(subclass: &str, matched_bits: Vec<&str>) -> PubChapter {
    let number_string = matched_bits[2].to_string();
    PubChapter {
        subclass: subclass.to_string(),
        number_string,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Milestone {
    subclass: String,
    s_or_e: Option<String>,
    tag_name: Option<String>,
    attributes: Vec<(String, String)>,
}

fn make_milestone(subclass: &str, matched_bits: Vec<&str>) -> Milestone {
    let mut ret = Milestone {
        subclass: subclass.to_string(),
        tag_name: None,
        s_or_e: None,
        attributes: Vec::new(),
    };

    if subclass != "endMilestoneMarker" {
        if matched_bits.len() > 2 {
            ret.tag_name = Some(matched_bits[2].to_string());
        }

        if subclass == "emptyMilestone" {
            if matched_bits.len() > 1 {
                let attributes_str = matched_bits[1];
                ret.attributes = attributes_str
                    .split('|')
                    .skip(1)
                    .filter_map(|attr| {
                        let parts: Vec<&str> = attr.split('=').collect();
                        if parts.len() == 2 {
                            let key = parts[0].to_string();
                            let value = parts[1]
                                .replace('"', "")
                                .replace('\\', "")
                                .replace('*', "");
                            Some((key, value))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        } else {
            if matched_bits.len() > 3 {
                ret.s_or_e = Some(matched_bits[3].to_string());
            }
        }
    }
    ret
}


#[derive(Debug, PartialEq, Clone)]
struct Tag {
    subclass: String,
    tag_name: String,
    is_nested: bool,
    tag_level: i32,
    full_tag_name: String,
    tag_type: String,
}

fn make_tag(subclass: &str, matched_bits: Vec<&str>) -> Tag {
    let mut tag_name = matched_bits[2].to_string();
    let is_nested = tag_name.starts_with('+');
    let tag_level: i32;
    let full_tag_name: String;
    if is_nested {
        tag_name = tag_name[1..].to_string();
    }
    if matched_bits[3] != "" {
        tag_level = matched_bits[3].parse::<i32>().unwrap();
    } else {
        tag_level = 1;
    }
    if matched_bits[3] == "1" {
        full_tag_name = tag_name.clone();
    } else {
        full_tag_name = format!("{}{}", tag_name, matched_bits[3]);
    }
    let char_marker = utils_usfm::char_markers();
    let para_marker = utils_usfm::para_markers();
    let note_markers = utils_usfm::note_markers();

    let tag_type: String;
    if char_marker.contains(&tag_name) {
        tag_type = "char".to_string();
    } else if para_marker.contains(&tag_name) {
        tag_type = "para".to_string();
    } else if tag_name == "id" {
        tag_type = "book".to_string();
    } else if note_markers.contains(&tag_name) {
        tag_type = "note".to_string();
    } else {
        // TODO : make sure to handle all the tags in the future
        // tag_type = "para".to_string();
        panic!("Tag not in the specification : {}", tag_name);
    }

    Tag {
        subclass: subclass.to_string(),
        tag_name,
        is_nested,
        tag_level,
        full_tag_name,
        tag_type,
    }
}


pub fn deserialize_from_file_usfm<T: AosjModel>(content: String) -> String {

    let mut regexes: Vec<(&str, &str, Regex)> = vec![];
    regexes.push(("chapter", r"([\r\n]*\\c[ \t]+(\d+)[ \t\r\n]*)", Regex::new(r"([\r\n]*\\c[ \t]+(\d+)[ \t\r\n]*)").unwrap()));
    regexes.push(("pubchapter", r"([\r\n]*\\cp[ \t]+([^\r\n]+)[ \t\r\n]*)", Regex::new(r"([\r\n]*\\cp[ \t]+([^\r\n]+)[ \t\r\n]*)").unwrap()));
    regexes.push(("verses", r"(\\v[ \t]+([\d\-]+)[ \t\r\n]*)", Regex::new(r"(\\v[ \t]+([\d\-]+)[ \t\r\n]*)").unwrap()));
    regexes.push(("attribute", r#"([ \t]*\|?[ \t]*([A-Za-z0-9\-]+)="([^"]*)"[ \t]?)"#, Regex::new(r#"([ \t]*\|?[ \t]*([A-Za-z0-9\-]+)="([^"]*)"[ \t]?)"#).unwrap()));
    regexes.push(("defaultAttribute", r"([ \t]*\|[ \t]*([^|\\]*))", Regex::new(r"([ \t]*\|[ \t]*([^|\\]*))").unwrap()));
    regexes.push(("emptyMilestone", r"(\\([a-z1-9]+)\\[*])", Regex::new(r"(\\([a-z1-9]+)\\[*])").unwrap()));
    regexes.push(("startMilestoneTag", r"(\\([a-z1-9]+)-([se]))", Regex::new(r"(\\([a-z1-9]+)-([se]))").unwrap()));
    regexes.push(("endMilestoneTag", r"(\\([*]))", Regex::new(r"(\\([*]))").unwrap()));
    regexes.push(("endTag", r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[*])", Regex::new(r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[*])").unwrap()));
    regexes.push(("startTag", r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[ \t]?)", Regex::new(r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[ \t]?)").unwrap()));
    regexes.push(("bareSlash", r"(\\)", Regex::new(r"(\\)").unwrap()));
    regexes.push(("quote", r#"(")"#, Regex::new(r#"(")"#).unwrap()));
    regexes.push(("eol", r"([ \t]*[\r\n]+[ \t]*)", Regex::new(r"([ \t]*[\r\n]+[ \t]*)").unwrap()));
    regexes.push(("noBreakSpace", r"~", Regex::new(r"~").unwrap()));
    regexes.push(("softLinebreak", r"//", Regex::new(r"//").unwrap()));
    regexes.push(("wordLike", r"([\p{L}\p{N}\p{M}\u2060]{1,127})", Regex::new(r"([\p{L}\p{N}\p{M}\u2060]{1,127})").unwrap()));
    regexes.push(("lineSpace", r"([\p{Zs}\t]{1,127})", Regex::new(r"([\p{Zs}\t]{1,127})").unwrap()));
    regexes.push(("punctuation", r"([\p{P}\p{Sm}\p{Sc}\p{Sk}\p{So}])", Regex::new(r"([\p{P}\p{Sm}\p{Sc}\p{Sk}\p{So}])").unwrap()));
    regexes.push(("unknown", r"(.)", Regex::new(r"(.)").unwrap()));

    let main_regex_str: String = regexes.iter().map(|&(_, v, _)| v).collect::<Vec<&str>>().join("|") + "/g";
    let _text_regex = [
        regexes.iter().find(|&&(k, _, _)| k == "wordLike").unwrap().1,
        regexes.iter().find(|&&(k, _, _)| k == "lineSpace").unwrap().1,
        regexes.iter().find(|&&(k, _, _)| k == "punctuation").unwrap().1,
        regexes.iter().find(|&&(k, _, _)| k == "eol").unwrap().1,
    ].join("|");
    let main_regex = Regex::new(&main_regex_str).unwrap();

    let mut tokens: Vec<Token> = vec![];

    for cap in main_regex.find_iter(&content) {
        for (subclass, _regex_str, regex) in &regexes {
            if let Some(matched_bits) = regex.captures(&cap.as_str()) {
                let matched_bits: Vec<&str> = matched_bits.iter().map(|m| m.map_or("", |m| m.as_str())).collect();
                let token = match *subclass {
                    "chapter" => Token::Chapter(make_chapter(subclass, matched_bits)),
                    "pubchapter" => Token::PubChapter(make_pub_chapter(subclass, matched_bits)),
                    "verses" => Token::Verses(make_verses(subclass, matched_bits)),
                    "attribute" | "defaultAttribute" => Token::Attribute(make_attribute(subclass, matched_bits)),
                    "emptyMilestone" | "startMilestoneTag" | "endMilestoneTag" => Token::Milestone(make_milestone(subclass, matched_bits)),
                    "startTag" | "endTag" => Token::Tag(make_tag(subclass, matched_bits)),
                    "bareSlash" | "quote" | "eol" | "noBreakSpace" | "softLinebreak" | "wordLike" | "lineSpace" | "punctuation" | "unknown" => Token::Printable(make_printable(subclass, matched_bits)),
                    _ => Token::Printable(make_bad(subclass, matched_bits)),
                };

                tokens.push(token);

                break;
            }
        }
    }

    let mut model = T::new();
    let mut txt: Vec<String> = Vec::new();
    let mut open_para_tags: Vec<Tag> = Vec::new();
    let mut open_char_tags: Vec<Tag> = Vec::new();
    let mut open_note_tags: Vec<Tag> = Vec::new();
    let mut attributes: BTreeMap<String, String> = BTreeMap::new();
    let mut in_start_char: bool = false;
    let mut in_milestone: bool = false;

    for token in tokens {
        match token {
            Token::Tag(t) => {

                match t.subclass.as_str() {
                    "startTag" => {
                        match t.tag_type.as_str() {
                            "para" => {
                                while open_char_tags.len() > 0 {
                                    let pop_tag = open_char_tags.pop().unwrap();
                                    do_end_tag(&mut model, pop_tag, &mut txt);
                                }
                                while open_para_tags.len() > 0 {
                                    let pop_tag = open_para_tags.pop().unwrap();
                                    do_end_tag(&mut model, pop_tag, &mut txt);
                                }
                                open_para_tags.push(t.clone());

                                let marker = t.full_tag_name;
                                attributes.insert("marker".to_string(), marker);
                                model.push_element(attributes.clone(), "para".to_string());
                                model.start_new_para(model.get_attributes());
                                attributes.clear();
                            }
                            "char" => {
                                match t.tag_name.as_str() {
                                    "w" => {
                                        if !txt.is_empty() {
                                            model.add_string_to_in_para(&mut txt);
                                        }
                                        in_start_char = true;
                                    }
                                    _ => {
                                        if !t.is_nested {
                                            while open_char_tags.len() > 0 {
                                                let pop_tag = open_char_tags.pop().unwrap();
                                                do_end_tag(&mut model, pop_tag, &mut txt);
                                            }
                                        }
                                        open_char_tags.push(t.clone());

                                        if !txt.is_empty() {
                                            model.add_string_to_in_para(&mut txt);
                                        }
                                        let marker = t.tag_name;
                                        attributes.insert("marker".to_string(), marker);

                                        model.push_element(attributes.clone(), "char".to_string());
                                        model.start_add_char_marker(model.get_attributes());
                                        attributes.clear();
                                        in_start_char = true;
                                    }
                                }
                            }
                            "note" => {
                                while open_char_tags.len() > 0 {
                                    let pop_tag = open_char_tags.pop().unwrap();
                                    do_end_tag(&mut model, pop_tag, &mut txt);
                                }
                                while open_note_tags.len() > 0 {
                                    let pop_tag = open_note_tags.pop().unwrap();
                                    do_end_tag(&mut model, pop_tag, &mut txt);
                                }
                                open_note_tags.push(t.clone());

                                let marker = t.full_tag_name;
                                attributes.insert("marker".to_string(), marker);
                                model.push_element(attributes.clone(), "note".to_string());
                                model.start_add_note(model.get_attributes());
                                attributes.clear();
                            }
                            "book" => {
                                open_para_tags.push(t.clone());
                            }

                            _ => {}
                        }
                    }
                    "endTag" => {
                        match t.tag_type.as_str() {
                            "char" => {
                                match t.tag_name.as_str() {
                                    "w" => {
                                        let marker = t.tag_name;
                                        attributes.insert("marker".to_string(), marker);
                                        model.push_element(attributes.clone(), "char".to_string());
                                        model.start_add_char_marker(model.get_attributes());
                                        if !txt.is_empty() {
                                            model.end_add_char_marker(&mut txt);
                                        }
                                        in_start_char = false;
                                        attributes.clear();
                                    }
                                    _ => {
                                        if !open_char_tags.is_empty() {
                                            let pop_tag = open_char_tags.pop().unwrap();
                                            do_end_tag(&mut model, pop_tag, &mut txt);
                                        }
                                    }
                                }
                            }
                            "note" => {
                                if !open_note_tags.is_empty() {
                                    let pop_tag = open_note_tags.pop().unwrap();
                                    do_end_tag(&mut model, pop_tag, &mut txt);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Token::Printable(p) => {

                if p.subclass != "eol" {
                    let re = Regex::new(r"[ \t\r\n]+").unwrap();
                    txt.push(re.replace_all(p.print_value.as_str(), " ").to_string());
                }
            }

            Token::Chapter(c) => {

                if !txt.is_empty() {
                    model.add_string_to_in_para(&mut txt);
                }

                while open_para_tags.len() > 0 {
                    let pop_tag = open_para_tags.pop().unwrap();
                    do_end_tag(&mut model, pop_tag, &mut txt);
                }

                let number = c.number_string;
                let marker = "c".to_string();
                attributes.insert("number".to_string(), number);
                attributes.insert("marker".to_string(), marker);
                model.push_element(attributes.clone(), "chapter".to_string());
                model.add_chapter(model.get_attributes());
                attributes.clear();
            }

            Token::Verses(v) => {

                if !txt.is_empty() {
                    model.add_string_to_in_para(&mut txt);
                }

                let number = v.number_string;
                let marker = "v".to_string();
                attributes.insert("number".to_string(), number);
                attributes.insert("marker".to_string(), marker);
                model.push_element(attributes.clone(), "verse".to_string());
                model.add_verse_to_in_para(model.get_attributes());
                attributes.clear();
            }

            Token::Milestone(m) => {

                if !txt.is_empty() {
                    model.add_string_to_in_para(&mut txt);
                }

                match m.subclass.as_str() {
                    "emptyMilestone" => {
                        let marker = m.tag_name.unwrap();
                        attributes.insert("marker".to_string(), marker);
                        model.push_element(attributes.clone(), "ms".to_string());
                        model.add_milestone(model.get_attributes());
                        attributes.clear();
                    }
                    "startMilestoneTag" => {
                        match m.s_or_e.unwrap().as_str() {
                            "s" => {
                                in_milestone = true;
                                let marker = format!("{}{}", m.tag_name.unwrap(), "-s");
                                attributes.insert("marker".to_string(), marker);
                            }
                            "e" => {
                                in_milestone = true;
                                let marker = format!("{}{}", m.tag_name.unwrap(), "-e");
                                attributes.insert("marker".to_string(), marker);
                            }
                            _ => {}
                        }
                    }
                    "endMilestoneTag" => {
                        if in_milestone {
                            model.push_element(attributes.clone(), "ms".to_string());
                            model.add_milestone(model.get_attributes());
                            attributes.clear();
                            in_milestone = false;
                        }
                    }
                    _ => {}
                }
            }

            Token::Attribute(a) => {
                attributes.insert(a.key, a.value_string);
            }

            _ => {}
        }
    }
    while open_char_tags.len() > 0 {
        let pop_tag = open_char_tags.pop().unwrap();
        do_end_tag(&mut model, pop_tag, &mut txt);
    }
    while open_para_tags.len() > 0 {
        let pop_tag = open_para_tags.pop().unwrap();
        do_end_tag(&mut model, pop_tag, &mut txt);
    }

    model.assemble_model()
}

fn do_end_tag<T: AosjModel>(model: &mut T, token: Tag, txt: &mut Vec<String>) {
    match token.tag_name.as_str() {
        "usfm" => {
            model.add_root_metadata(&txt.join(""));
            txt.clear();
        }
        _ => {
            match token.tag_type.as_str() {
                "book" => {
                    let code = txt.clone().join("").chars().take(3).collect::<String>();
                    let marker = token.tag_name;
                    let mut attributes: BTreeMap<String, String> = BTreeMap::new();

                    attributes.insert("code".to_string(), code);
                    attributes.insert("marker".to_string(), marker);
                    model.push_element(attributes, "book".to_string());
                    model.start_book(model.get_attributes());
                    model.add_string_to_in_para(&mut vec![txt.clone().join("")[4..].to_string()]);
                    model.end_book();
                    txt.clear();
                }

                "para" => {
                    model.add_string_to_in_para(txt);
                    model.end_new_para();
                }

                "char" => {
                    model.end_add_char_marker(txt);
                }

                "note" => {
                    model.end_add_note(txt);
                }
                _ => {}
            }
        }
    }
}

pub fn deserialize_from_file_path_usfm<T:AosjModel>(input_file_path: &str) -> String {
    let mut file = File::open(input_file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    deserialize_from_file_usfm::<T>(content)
}

pub fn deserialize_from_file<T:AosjModel>(content: String) -> String {
    deserialize_from_file_usfm::<T>(content)
}
