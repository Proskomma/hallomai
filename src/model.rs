#![allow(dead_code)]

use std::println;
use serde::Deserialize;
// use std::collections::HashMap;
// use serde::{Deserialize, Deserializer, Serialize};
// use serde::de::Error;
// use serde_with::skip_serializing_none;
use crate::deserialize_usj::USJ;
use crate::deserialize_usx::USX;

// #[skip_serializing_none]
// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum InParaObject {
//     Verse(VerseObject),
//     Char(CharMarkerObject),
//     Milestone(MilestoneObject),
//     Figure(FigureObject),
//     Note(NoteObject),
//     String(String),
// }
//
//
// #[skip_serializing_none]
// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(tag = "type")]
// #[serde(rename_all = "camelCase")]
// pub enum Content {
//     Book(BookObject),
//     Chapter(ChapterObject),
//     Para(ParaMarkerObject),
//     Table(TableObject),
//     Sidebar(SidebarObject),
// }

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
// #[serde(untagged)]
pub enum DataType {
    USJ(USJ),
    USX(USX)
}

pub trait DataTypeTraits {
    fn print_version(&self);
    fn print_type(&self);
    fn print_content(&self);

    fn get_version(&self);
    fn get_type(&self);
    fn get_content(&self);
}

impl DataTypeTraits for DataType {
    fn print_version(&self) {
        match self {
            DataType::USJ(data) => {
                println!("{}", data.version)
            }
            DataType::USX(data) => {
                println!("{}", data.version)
            }
        }
    }

    fn print_type(&self) {
        match self {
            DataType::USJ(data) => {
                println!("{}", data.r#type)
            }
            DataType::USX(data) => {
                println!("{}", data.r#type)
            }
        }
    }

    fn print_content(&self) {
        match self {
            DataType::USJ(data) => {
                println!("{}", data.content)
            }
            DataType::USX(data) => {
                println!("{}", data.content)
            }
        }
    }

    fn get_version(&self) {
        match self {
            DataType::USJ(data) => {
                println!("{}", data.content)
            }
            DataType::USX(data) => {
                println!("{}", data.content)
            }
        }
    }

    fn get_type(&self) {
        todo!()
    }

    fn get_content(&self) {
        todo!()
    }
}
