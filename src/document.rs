use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(file: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(&file)?;
        let mut rows = Vec::new();

        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self { rows })
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
}
