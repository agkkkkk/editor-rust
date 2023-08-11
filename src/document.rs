use crate::CursorPosition;
use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(file: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(&file)?;
        let mut rows = Vec::new();

        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            rows,
            file_name: Some(file.to_string()),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn length(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at_posi: &CursorPosition, character: char) {
        if at_posi.y == self.length() {
            let mut row = Row::default();
            row.insert(0, character);
            self.rows.push(row);
        } else if at_posi.y < self.length() {
            let row = self.rows.get_mut(at_posi.y).unwrap();
            row.insert(at_posi.x, character);
        }
    }

    pub fn delete(&mut self, at_posi: &CursorPosition) {
        if at_posi.y >= self.length() {
            return;
        } else {
            let row = self.rows.get_mut(at_posi.y).unwrap();
            row.delete(at_posi.x);
        }
    }

    pub fn append(&mut self, line: Row) {
        
    }
}
