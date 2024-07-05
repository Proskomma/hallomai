#![allow(dead_code)]
use crate::structs_model;

pub enum InParaObject {
    Verse(structs_model::verse_object::VerseObject),
    Char(structs_model::char_marker_object::CharMarkerObject),
    Milestone(structs_model::milestone_object::MilestoneObject),
    Figure(structs_model::figure_object::FigureObject),
    Note(structs_model::note_object::NoteObject),
    String(String),
}


pub enum Content {
    Book(structs_model::book_object::BookObject),
    Chapter(structs_model::chapter_object::ChapterObject),
    Para(structs_model::para_marker_object::ParaMarkerObject),
    Table(structs_model::table_object::TableObject),
    Sidebar(structs_model::sidebar_object::SidebarObject),
}
