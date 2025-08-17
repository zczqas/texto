#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    // By using let editor... before, we told Rust that we want to have a read-only reference to Editor.
    // However, editor.run() now modifies the Editor.
    // We could make this an explicit reference that we intend to modify - or we could sidestep it by eliminating the variable editor and calling run() directly
    // on the output of default().

    // You can also store editor as a mutable reference
    // let mut editor = Editor.default();
    // editor.run();
    Editor::default().run();
}
