#![warn(clippy::all, clippy::pedantic)]
use std::convert::From;
use std::fs;

pub struct Row {
    pub string: String,
}

impl From<&str> for Row {
    fn from(string: &str) -> Self {
        Row {
            string: string.into(),
        }
    }
}

#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        Ok(Self {
            rows: content.lines().map(|s| s.into()).collect(),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
