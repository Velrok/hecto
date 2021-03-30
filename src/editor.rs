#![warn(clippy::all, clippy::pedantic)]
use crate::document::Row;
use crate::Document;
use crate::Terminal;
// use std::io;
// use std::io::stdout;
use std::cmp::{max, min};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            document: Document::open(),
            terminal: Terminal::default().expect("Failed to init terminal."),
            cursor_position: Position::default(),
        }
    }

    fn move_cursor(&mut self, dx: isize, dy: isize) {
        let Position { x, y } = self.cursor_position;
        let crate::terminal::Size { width, height } = self.terminal.size();

        let nx = min(max(0, x as isize + dx), *width as isize - 1);
        let ny = min(max(0, y as isize + dy), *height as isize - 1);

        self.cursor_position = Position {
            x: nx as usize,
            y: ny as usize,
        };
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char('j') => self.move_cursor(0, 1),
            Key::Char('k') => self.move_cursor(0, -1),
            Key::Char('h') => self.move_cursor(-1, 0),
            Key::Char('l') => self.move_cursor(1, 0),
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

        for i in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.get_row(i as usize) {
                println!("{}\r", row.string);
            } else {
                println!("~\r");
            }
        }
    }

    fn get_row(&self, i: usize) -> Option<&Row> {
        self.document.rows.get(i)
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
            Terminal::cursor_position(self.cursor_position.x as u16, self.cursor_position.y as u16);
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
