#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;

use document::Document;
use editor::Editor;
use row::Row;
use terminal::Terminal;
use editor::Position;

fn main() {
    Editor::default().run();
}
