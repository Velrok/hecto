use std::io;
use std::io::stdout;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let (width, height) = termion::terminal_size()?;
        Ok(Self {
            size: Size { width, height },
            // There are a few things to note here. First, we are using termion to provide stdout, the counterpart of stdin from above with a function called into_raw_mode(), which we are calling. But why are we calling that method on stdout to change how we read from stdin? The answer is that terminals have their states controlled by the writer, not the reader. The writer is used to draw on the screen or move the cursor, so it is also used to change the mode as well.
            // RAW mode
            // 1. no line buffering
            // 2. input is not echoed
            // 3. output is not cannonicallized (no carrage return with new line)
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        print!(
            "{}",
            // saturating_add: prevent u16 overflow
            // by returning max_value if blown
            termion::cursor::Goto(x.saturating_add(1), y.saturating_add(1))
        );
    }

    // needs a raw terminal mode term
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            // next will not block it returns Some if something was pressed or None if nothing was
            // pressed
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
            // if None we loop ie busy wait for actual input
        }
    }
}
