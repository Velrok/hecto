// Based on https://www.philippflenker.com/hecto-chapter-2/

mod editor;

use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
