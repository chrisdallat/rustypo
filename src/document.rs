use crate::Position;
use crate::Row;
use std::fs::{self, File};
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {

    pub fn open(filename: &str) -> Result<Self, std::io::Error> {

        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self {
            rows, 
            filename: Some(filename.to_string()),
        })

    }

    pub fn save(_filename: &str) -> Result<() , std::io::Error> {
        let message: String = "hello world!".to_string();
        let mut _new_file = fs::File::create("new_file.txt").unwrap();
        Ok({
            fs::write("new_file.txt", message).unwrap();
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        } 
        let row = self.rows.get_mut(at.y).unwrap();
        row.delete(at.x);
    }

}

