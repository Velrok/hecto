// Based on https://www.philippflenker.com/hecto-chapter-2/

mod document;
mod editor;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
