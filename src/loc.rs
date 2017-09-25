use std::io::BufRead;

pub fn count_lines_of_code<R: BufRead>(input: R) -> usize {
    input.lines()
        // TODO: error handling
        .map(|result| result.unwrap())
        .filter(|line| line.trim().len() != 0)
        .count()
}

mod test {
    #[test]
    fn counts_lines() {
        use std::io::Cursor;
        let cursor = Cursor::new(b"lorem\nipsum\r\ndolor");
        let loc = super::count_lines_of_code(cursor);
        assert_eq!(loc, 3);
    }

    #[test]
    fn ignores_empty_lines() {
        use std::io::Cursor;
        let cursor = Cursor::new(b"lorem\n \r\ndolor");
        let loc = super::count_lines_of_code(cursor);
        assert_eq!(loc, 2);
    }
}