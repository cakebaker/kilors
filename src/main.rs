#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
mod document;
mod editor;
mod highlighting;
mod row;
mod terminal;

use document::Document;
use editor::Editor;
use row::Row;
use terminal::Terminal;
use editor::Position;
use editor::SearchDirection;

fn main() {
    Editor::default().run();
}
