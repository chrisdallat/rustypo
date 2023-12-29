use std::io::{self, stdin, stdout};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;


pub struct Editor {
    should_quit: bool,
}

impl Editor {

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error)
            } 
            if self.should_quit {
                println!("Program Exited!\r");
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self {should_quit: false}
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
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

fn die(error: &std::io::Error) {
    panic!("{}", error);
}