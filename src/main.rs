mod editor;
use editor::Editor;
fn main() {
    let _editor = Editor::default();
    print!("\x1b[2J");
    _editor.draw_rows();
    _editor.run();
}
