#![warn(clippy::all)]
mod editor;

use editor::Editor;

fn main() {
    println!("Starting Rustypo, Enter Text...");  

    let editor = Editor::default();
    editor.run();
}

