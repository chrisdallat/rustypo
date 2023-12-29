use std::io::{stdin, stdout};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;


pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?} \r", c as u8); 
                        } else {
                            println!("{:?} ({})\r", c as u8, c); 
                        }
                    }
                    Key::Ctrl('c') => break,
                    _ => println!("{key:?} \r"),
                },
                Err(err) => die(&err),
            }
        } 
    }

    pub fn default() -> Self {
        Self {}
    }

}
    


fn die(e: &std::io::Error) {
    panic!("{}", e);
}