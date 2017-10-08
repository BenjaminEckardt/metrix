pub fn loc(input: String) -> usize {
    input.lines()
        //.filter_map(|result| result.ok())
        .map(|line| line.trim().to_string())
        .filter(|trimmed_line| trimmed_line.len() != 0)
        .filter(|trimmed_line| !trimmed_line.starts_with("//"))
        .count()
}

#[cfg(test)]
mod test {
    #[test]
    fn counts_lines() {
        let cursor = String::from("counts\ncounts\r\ncounts");
        let loc = super::loc(cursor);
        assert_eq!(loc, 3);
    }

    #[test]
    fn ignores_empty_lines() {
        let cursor = String::from("counts\n \r\ncounts");
        let loc = super::loc(cursor);
        assert_eq!(loc, 2);
    }

    #[test]
    fn ignores_single_line_comments() {
        let cursor = String::from("counts\ncounts//\n//doesnt");
        let loc = super::loc(cursor);
        assert_eq!(loc, 2);
    }
}
