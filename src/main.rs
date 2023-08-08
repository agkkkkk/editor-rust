mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
pub use editor::CursorPosition;
pub use editor::Editor;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
