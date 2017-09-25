extern crate glob;
extern crate prettytable;

mod loc;

use std::fs::File;
use std::io::BufReader;
use glob::glob;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

fn main() {
    let mut t = Table::new();
    t.add_row(Row::new(vec![Cell::new("path"), Cell::new("loc")]));
    let table = glob(&"./**/*.rs").unwrap().into_iter()
        .map(|path_result| path_result.unwrap())
        .filter(|path| !path.is_dir())
        .map(|path| {
            let file = File::open(path.clone()).unwrap();
            let buf_reader = BufReader::new(file);
            (path, loc::count_lines_of_code(buf_reader))
        }).fold(t, |mut result_table, path_and_count| {
            let path = &path_and_count.0.display().to_string();
            let count = &path_and_count.1.to_string();
            result_table.add_row(Row::new(vec![Cell::new(path), Cell::new(count)]));
            result_table
        });

    table.printstd();
}
