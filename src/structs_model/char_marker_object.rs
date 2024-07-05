#![allow(dead_code)]
use crate::structs_model::model::InParaObject;

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
