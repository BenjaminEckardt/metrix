use std::io::BufRead;
use std::io::Cursor;

fn main() {
    let cursor = Cursor::new(b"lorem\nipsum\r\ndolor");
    count_lines(cursor)
}

fn count_lines<R: BufRead>(input: R) {
    println!("{}", input.lines().count())
}
