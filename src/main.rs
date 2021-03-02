// Based on https://www.philippflenker.com/hecto-chapter-2/
use std::io;
use std::io::stdout;
use std::io::Read;
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8; // 8 bit

    // ASCII codes 0–31 are all control characters, and 127 is also a control character. ASCII codes 32–126 are all printable. (Check out the ASCII table to see all of the characters.)
    byte & 0b0001_1111 // last 5 -> 0..31
}

fn die(e: std::io::Error) {
    panic!(e)
}

fn main() {
    println!(">>>");

    // There are a few things to note here. First, we are using termion to provide stdout, the counterpart of stdin from above with a function called into_raw_mode(), which we are calling. But why are we calling that method on stdout to change how we read from stdin? The answer is that terminals have their states controlled by the writer, not the reader. The writer is used to draw on the screen or move the cursor, so it is also used to change the mode as well.
    // RAW mode
    // 1. no line buffering
    // 2. input is not echoed
    // 3. output is not cannonicallized (no carrage return with new line)
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    println!("{:#b} \r", b);
                } else {
                    println!("{:#b} ({})\r", b, c);
                }

                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => die(e),
        }
    }
}
