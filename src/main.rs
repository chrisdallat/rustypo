mod editor;

use editor::Editor;

fn main() {
    println!("Starting Rustypo, Enter Text...");  

    let editor = Editor {};
    editor.run();
}

