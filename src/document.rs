#![warn(clippy::all, clippy::pedantic)]
use std::convert::From;

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
    pub fn open() -> Self {
        Self {
            rows: vec!["hello world".into()],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
