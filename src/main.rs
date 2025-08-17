mod editor;
// This is how we make our new module - editor - known to the current module. It looks for a file called editor.rs or one called editor/mod.rs.

use editor::Editor;

fn main() {
    let editor = Editor::default();

    editor.run();
}
