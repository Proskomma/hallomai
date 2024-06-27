#![allow(dead_code)]
use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InParaObject {
    Verse(VerseObject),
    Char(CharMarkerObject),
    Milestone(MilestoneObject),
    Figure(FigureObject),
    Note(NoteObject),
    String(String),
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Content {
    Book(BookObject),
    Chapter(ChapterObject),
    Para(ParaMarkerObject),
    Table(TableObject),
    Sidebar(SidebarObject),
}



#[skip_serializing_none]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub(crate) r#type: String,
    pub(crate) version: String,
    pub(crate) content: String,
}

const INDEX_DEFAULT: usize = 0;
fn index_default() -> usize {
    INDEX_DEFAULT
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "para")]
#[serde(tag = "type")]
pub struct ParaMarkerObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) content: Option<Vec<InParaObject>>,
}

impl ParaMarkerObject {
    pub fn init_index(&mut self) {
        let mut tot_len: usize = 0;
        if self.marker.eq("b") && self.content.is_none() {
            self.index = 1;
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "table")]
#[serde(tag = "type")]
pub struct TableObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "sidebar")]
#[serde(tag = "type")]
pub struct SidebarObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "char")]
#[serde(tag = "type")]
pub struct CharMarkerObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "note")]
#[serde(tag = "type")]
pub struct NoteObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "row")]
#[serde(tag = "type")]
pub struct RowObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "cell")]
#[serde(tag = "type")]
pub struct CellObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "book")]
#[serde(tag = "type")]
pub struct BookObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "chapter")]
#[serde(tag = "type")]
pub struct ChapterObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "verse")]
#[serde(tag = "type")]
pub struct VerseObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "ms")]
#[serde(tag = "type")]
pub struct MilestoneObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
    pub(crate) index: usize,
    pub(crate) marker: String,
    pub(crate) who: Option<String>,
    pub(crate) eid: Option<String>,
    #[serde(flatten)]
    pub(crate) additional_properties: Option<HashMap<String, String>>,
}

impl MilestoneObject {
    pub fn init_index(&mut self) {
        self.index = 0;
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "figure")]
#[serde(tag = "type")]
pub struct FigureObject {
    #[serde(skip_serializing)]
    #[serde(default = "index_default")]
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
