#![allow(dead_code)]
use crate::structs_model::cell_object::CellObject;
use crate::structs_model::model::InParaObject;

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
