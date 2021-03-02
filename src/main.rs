use std::io;
use std::io::Read;
use std::io::stdout;
use termion::raw::IntoRawMode;

fn main() {
    println!(">>>");
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap() as char;
        let c = b as char;

        if c.is_control() {
            println!("{:?} \r", c);
        } else {
            println!("{:?} ({})\r", b, c);
        }

        if c == 'q' { break; }
    }
}
