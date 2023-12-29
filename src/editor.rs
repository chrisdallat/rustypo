use std::io::{self, stdin, stdout};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;


pub struct Editor {}

impl Editor {

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error)
            } 
        }
    }

    pub fn default() -> Self {
        Self {}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('c') => panic!("\r\nProgram Exited!\r\n"),
            _ => {
                println!("process_keypress Ok({:?})\r", pressed_key);
            },
        }
        Ok(())
    }

}
    
fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}