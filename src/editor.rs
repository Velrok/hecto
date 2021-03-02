#![warn(clippy::all, clippy::pedantic)]
use std::io;
use std::io::stdout;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    }

    pub fn run(&self) {
        // There are a few things to note here. First, we are using termion to provide stdout, the counterpart of stdin from above with a function called into_raw_mode(), which we are calling. But why are we calling that method on stdout to change how we read from stdin? The answer is that terminals have their states controlled by the writer, not the reader. The writer is used to draw on the screen or move the cursor, so it is also used to change the mode as well.
        // RAW mode
        // 1. no line buffering
        // 2. input is not echoed
        // 3. output is not cannonicallized (no carrage return with new line)
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        // next will not block it returns Some if something was pressed or None if nothing was
        // pressed
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
        // TODO: Finish chapter 3 -> https://www.philippflenker.com/hecto-chapter-3/
        // if None we loop ie busy wait for actual input
    }
}

fn die(e: std::io::Error) {
    panic!(e)
}
