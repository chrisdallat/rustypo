#![warn(clippy::all)]
mod document;
mod rows;
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position; 
pub use document::Document;
pub use rows::Row;

fn main() {
    println!("Starting Rustypo, Enter Text...");  

    Editor::default().run();
}

