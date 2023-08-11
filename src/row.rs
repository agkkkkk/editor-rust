use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),

            len: 0,
        };

        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        let mut result = String::new();

        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme)
            }
        }

        result
    }

    pub fn length(&self) -> usize {
        self.len
    }

    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, at: usize, character: char) {
        if at >= self.length() {
            self.string.push(character);
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let second_half: String = self.string[..].graphemes(true).skip(at).collect();

            result.push(character);
            result.push_str(&second_half);

            self.string = result;
        }

        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.length() {
            return;
        } else {
            let mut first_half: String = self.string[..].graphemes(true).take(at).collect();
            let second_half: String = self.string[..].graphemes(true).skip(at + 1).collect();

            first_half.push_str(&second_half);

            self.string = first_half;
        }

        self.update_len();
    }
}
