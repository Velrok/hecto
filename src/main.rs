// Based on https://www.philippflenker.com/hecto-chapter-2/

mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
