use std::io::Cursor;

mod loc;

fn main() {
    let cursor = Cursor::new(b"lorem\nipsum\r\ndo\nlor");
    loc::count_lines_of_code(cursor);
}
