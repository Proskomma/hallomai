#![allow(dead_code)]
use crate::structs_model::model::InParaObject;
use crate::structs_model::para_marker_object::ParaMarkerObject;

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
