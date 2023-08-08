mod editor;
mod terminal;
pub use editor::CursorPosition;
use editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default().run()
}
