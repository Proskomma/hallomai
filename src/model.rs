#![allow(dead_code)]
use std::collections::HashMap;
// use serde::{Deserialize, Deserializer, Serialize};
// use serde::de::Error;
// use serde_json::Result;
// use serde_json::Value;
// use serde_with::skip_serializing_none;

pub enum InParaObject {
    Verse(VerseObject),
    Char(CharMarkerObject),
    Milestone(MilestoneObject),
    Figure(FigureObject),
    Note(NoteObject),
    String(String),
}


pub enum Content {
    Book(BookObject),
    Chapter(ChapterObject),
    Para(ParaMarkerObject),
    Table(TableObject),
    Sidebar(SidebarObject),
}

// const Root_DEFAULT: String = "usx".into();
fn usj_default() -> String {
    "usx".to_string()
}

pub struct Root {
    pub(crate) r#type: String,
    pub(crate) version: String,
    pub(crate) content: Vec<Content>,
}

impl Root {
    pub fn add_book(&mut self, book: BookObject) {
        self.content.push(Content::Book(book));
    }

    pub fn set_version(&mut self, new_version: &str) {
        self.version = new_version.to_string();
    }
}

const INDEX_DEFAULT: usize = 0;
fn index_default() -> usize {
    INDEX_DEFAULT
}

pub struct ParaMarkerObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<InParaObject>>,
}

impl ParaMarkerObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        if self.marker.eq("b") && self.content.is_none() {
            self.index = 1;
            // tot_len += 1;
            return;
        }
        for in_para_obj in self.content.iter_mut().flatten() {
            match in_para_obj {
                InParaObject::String(st) => {
                    tot_len += st.chars().collect::<Vec<_>>().len();
                }
                InParaObject::Verse(verse) => {
                    verse.init_index();
                    tot_len += verse.index;
                }
                InParaObject::Char(char) => {
                    char.init_index();
                    tot_len += char.index;
                }
                InParaObject::Milestone(ms) => {
                    tot_len += ms.index;
                }
                InParaObject::Figure(fig) => {
                    tot_len += fig.index;
                }
                InParaObject::Note(nt) => {
                    tot_len += nt.index;
                }
            }
        }
        self.index = tot_len;
    }
}

pub struct TableObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<RowObject>>,
}

impl TableObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for row in self.content.iter_mut().flatten() {
            for cell in row.content.iter_mut().flatten() {
                for in_para_obj in cell.content.iter_mut().flatten() {
                    match in_para_obj {
                        InParaObject::String(st) => {
                            tot_len += st.chars().collect::<Vec<_>>().len();
                        }
                        InParaObject::Verse(verse) => {
                            verse.init_index();
                            tot_len += verse.index;
                        }
                        InParaObject::Char(char) => {
                            char.init_index();
                            tot_len += char.index;
                        }
                        InParaObject::Milestone(ms) => {
                            tot_len += ms.index;
                        }
                        InParaObject::Figure(fig) => {
                            tot_len += fig.index;
                        }
                        InParaObject::Note(nt) => {
                            tot_len += nt.index;
                        }
                    }
                }
            }
        }
        self.index = tot_len;
    }
}

pub struct SidebarObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option< Vec<ParaMarkerObject> >,
    pub(crate) category: Option<String>,
}

impl SidebarObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        if self.marker.eq("b") {
            tot_len += 1;
        }
        for para_marker_obj in self.content.iter_mut().flatten() {
            for in_para_object in para_marker_obj.content.iter_mut().flatten() {
                match in_para_object {
                    InParaObject::String(st) => {
                        tot_len += st.chars().collect::<Vec<_>>().len();
                    }
                    InParaObject::Verse(verse) => {
                        verse.init_index();
                        tot_len += verse.index;
                    }
                    InParaObject::Char(char) => {
                        char.init_index();
                        tot_len += char.index;
                    }
                    InParaObject::Milestone(ms) => {
                        tot_len += ms.index;
                    }
                    InParaObject::Figure(fig) => {
                        tot_len += fig.index;
                    }
                    InParaObject::Note(nt) => {
                        tot_len += nt.index;
                    }
                }
            }
        }
        self.index = tot_len;
    }
}

pub struct CharMarkerObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<InParaObject>>,
    pub(crate) link_id: Option<String>,
    pub(crate) link_href: Option<String>,
    pub(crate) srcloc: Option<String>,
    pub(crate) strong: Option<String>,
}

impl CharMarkerObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for in_para_obj in self.content.iter_mut().flatten() {
            match in_para_obj {
                InParaObject::String(st) => {
                    tot_len += st.chars().collect::<Vec<_>>().len();
                }
                InParaObject::Verse(verse) => {
                    verse.init_index();
                    tot_len += verse.index;
                }
                InParaObject::Char(char) => {
                    char.init_index();
                    tot_len += char.index;
                }
                InParaObject::Milestone(ms) => {
                    ms.init_index();
                    tot_len += ms.index;
                }
                InParaObject::Figure(fig) => {
                    fig.init_index();
                    tot_len += fig.index;
                }
                InParaObject::Note(nt) => {
                    nt.init_index();
                    tot_len += nt.index;
                }
            }
        }
        self.index = tot_len;
    }
}


pub struct NoteObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<InParaObject>>,
    pub(crate) caller: Option<String>,
}

impl NoteObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for in_para_obj in self.content.iter_mut().flatten() {
            match in_para_obj {
                InParaObject::String(st) => {
                    tot_len += st.chars().collect::<Vec<_>>().len();
                }
                InParaObject::Verse(verse) => {
                    verse.init_index();
                    tot_len += verse.index;
                }
                InParaObject::Char(char) => {
                    char.init_index();
                    tot_len += char.index;
                }
                InParaObject::Milestone(ms) => {
                    tot_len += ms.index;
                }
                InParaObject::Figure(fig) => {
                    tot_len += fig.index;
                }
                InParaObject::Note(nt) => {
                    tot_len += nt.index;
                }
            }
        }
        self.index = tot_len;
    }
}

pub struct RowObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<CellObject>>,
}

impl RowObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for cell in self.content.iter_mut().flatten() {
            for in_para_obj in cell.content.iter_mut().flatten() {
                match in_para_obj {
                    InParaObject::String(st) => {
                        tot_len += st.chars().collect::<Vec<_>>().len();
                    }
                    InParaObject::Verse(verse) => {
                        verse.init_index();
                        tot_len += verse.index;
                    }
                    InParaObject::Char(char) => {
                        char.init_index();
                        tot_len += char.index;
                    }
                    InParaObject::Milestone(ms) => {
                        tot_len += ms.index;
                    }
                    InParaObject::Figure(fig) => {
                        tot_len += fig.index;
                    }
                    InParaObject::Note(nt) => {
                        tot_len += nt.index;
                    }
                }
            }
        }
        self.index = tot_len;
    }
}

pub struct CellObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<InParaObject>>,
    pub(crate) align: Option<String>,
    pub(crate) colspan: Option<usize>,
}

impl CellObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for in_para_obj in self.content.iter_mut().flatten() {
            match in_para_obj {
                InParaObject::String(st) => {
                    tot_len += st.chars().collect::<Vec<_>>().len();
                }
                InParaObject::Verse(verse) => {
                    verse.init_index();
                    tot_len += verse.index;
                }
                InParaObject::Char(char) => {
                    char.init_index();
                    tot_len += char.index;
                }
                InParaObject::Milestone(ms) => {
                    tot_len += ms.index;
                }
                InParaObject::Figure(fig) => {
                    tot_len += fig.index;
                }
                InParaObject::Note(nt) => {
                    tot_len += nt.index;
                }
            }
        }
        self.index = tot_len;
    }
}

pub struct BookObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) code: String,
    pub(crate) content: Option<Vec<String>>,
}

impl BookObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        for strs in self.content.iter().flatten() {
            tot_len += strs.chars().collect::<Vec<_>>().len();
        }
        self.index = tot_len;
    }

    pub fn get(&self) -> &BookObject {
        self
    }
}

pub struct ChapterObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) number: String,
    pub(crate) altnumber: Option<String>,
    pub(crate) pubnumber: Option<String>,
}

impl ChapterObject {
    pub fn init_index(&mut self) {
        self.index = self.number.chars().collect::<Vec<_>>().len();
    }
}

fn calculate_verse_index(number: &String) -> usize {
    number.chars().collect::<Vec<_>>().len()
}

/*impl<'de> Deserialize<'de> for VerseObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VerseObjectHelper {
            index: usize,
            marker: String,
            number: String,
            altnumber: Option<String>,
            pubnumber: Option<String>,
        }

        let helper = VerseObjectHelper::deserialize(deserializer)?;

        // Check if the marker is "v"
        if helper.marker != "v" {
            return Err(D::Error::custom("Invalid marker value"));
        }

        // let index = calculate_verse_index(&helper.number);

        Ok(VerseObject {
            index: "0".parse().unwrap(),
            marker: helper.marker,
            number: helper.number,
            altnumber: helper.altnumber,
            pubnumber: helper.pubnumber,
        })
    }
}
*/
pub struct VerseObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) number: String,
    pub(crate) altnumber: Option<String>,
    pub(crate) pubnumber: Option<String>,
}
impl VerseObject {
    pub fn init_index(&mut self) {
        let num = &self.number;
        self.index = num.chars().collect::<Vec<_>>().len();
    }

    pub fn get_index(self) -> usize {
        self.index
    }
}

pub struct MilestoneObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) who: Option<String>,
    pub(crate) additional_properties: Option<HashMap<String, String>>,
}

impl MilestoneObject {
    pub fn init_index(&mut self) {
        self.index = 0;
    }
}

pub struct FigureObject {
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<String>>,
    pub(crate) file: Option<String>,
    pub(crate) size: Option<String>,
    pub(crate) r#ref: Option<String>,
}

impl FigureObject {
    pub fn init_index(&mut self) {
        self.index = 0;
    }
}
