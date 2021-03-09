#![warn(clippy::all, clippy::pedantic)]
use crate::Terminal;
// use std::io;
// use std::io::stdout;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to init terminal."),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_welcome_msg(&self) {
        let welcome_msg = format!("Hecto -- version {}!", VERSION);
        let to = std::cmp::min(self.terminal.size().width as usize, welcome_msg.len());

        let height = self.terminal.size().height;
        let width = self.terminal.size().width;
        Terminal::cursor_position(width / 3, height / 3);
        println!("{}", &welcome_msg[..to]);
        Terminal::cursor_position(0, 0);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for _ in 0..height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            self.draw_welcome_msg();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn run(&mut self) {
        loop {
            // clear screen
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            // exit here after cleaning if we have to
            if self.should_quit {
                break;
            }
            // only process new stuff if need be
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!(e)
}
