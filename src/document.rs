use crate::CursorPosition;
use crate::Row;
use std::fs;
use std::io::{Error, Write};

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

    pub fn insert_new_line(&mut self, at_posi: &CursorPosition) {
        if at_posi.y > self.length() {
            return;
        }

        if at_posi.y > self.length() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at_posi.y).unwrap().split(at_posi.x);
        self.rows.insert(at_posi.y + 1, new_row);
    }

    pub fn insert(&mut self, at_posi: &CursorPosition, character: char) {
        if character == '\n' {
            self.insert_new_line(at_posi);
            return;
        }

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
        let len = self.length();
        if at_posi.y >= len {
            return;
        }
        if at_posi.x == self.rows.get_mut(at_posi.y).unwrap().length() && at_posi.y < len - 1 {
            let next_row = self.rows.remove(at_posi.y + 1);
            let row = self.rows.get_mut(at_posi.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at_posi.y).unwrap();
            row.delete(at_posi.x);
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(&file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }

        Ok(())
    }
}
