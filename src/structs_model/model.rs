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

pub struct Model {
    book: structs_model::book_object::BookObject,
    chapter: structs_model::chapter_object::ChapterObject,
    para: structs_model::para_marker_object::ParaMarkerObject,
    table: structs_model::table_object::TableObject,
    sidebar: structs_model::sidebar_object::SidebarObject,
    verse: structs_model::verse_object::VerseObject,
    char: structs_model::char_marker_object::CharMarkerObject,
    milestone: structs_model::milestone_object::MilestoneObject,
    figure: structs_model::figure_object::FigureObject,
    note: structs_model::note_object::NoteObject,
    string: String,
}