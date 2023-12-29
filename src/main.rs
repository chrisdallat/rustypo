#![warn(clippy::all)]
mod editor;

use editor::Editor;

fn main() {
    println!("Starting Rustypo, Enter Text...");  

    Editor::default().run();
}

