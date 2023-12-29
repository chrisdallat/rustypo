#![warn(clippy::all)]
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal; 
pub use editor::Position;

fn main() {
    println!("Starting Rustypo, Enter Text...");  

    Editor::default().run();
}

