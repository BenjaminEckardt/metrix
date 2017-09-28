use std::io::BufRead;

pub fn loc<R: BufRead>(input: R) -> usize {
    input.lines()
        // TODO: error handling
        .map(|result| result.unwrap())
        .map(|line| line.trim().to_string())
        .filter(|trimmed_line| trimmed_line.len() != 0)
        .filter(|trimmed_line| !trimmed_line.starts_with("//"))
        .count()
}

mod test {
    #[test]
    fn counts_lines() {
        use std::io::Cursor;
        let cursor = Cursor::new(b"counts\ncounts\r\ncounts");
        let loc = super::loc(cursor);
        assert_eq!(loc, 3);
    }

    #[test]
    fn ignores_empty_lines() {
        use std::io::Cursor;
        let cursor = Cursor::new(b"counts\n \r\ncounts");
        let loc = super::loc(cursor);
        assert_eq!(loc, 2);
    }

    #[test]
    fn ignores_single_line_comments() {
        use std::io::Cursor;
        let cursor = Cursor::new(b"counts\ncounts//\n//doesnt");
        let loc = super::loc(cursor);
        assert_eq!(loc, 2);
    }
}