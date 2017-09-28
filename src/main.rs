extern crate gitignore;
extern crate prettytable;

mod metrics;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

fn main() {
    let project_source_paths = determine_realtive_source_paths();
    let path_and_loc = read_files_and_count_loc(project_source_paths);
    let result_table = create_table(path_and_loc);
    result_table.printstd();
}

fn read_files_and_count_loc(project_source_paths: Vec<PathBuf>) -> Vec<(PathBuf, usize)> {
    let mut path_and_loc: Vec<(PathBuf, usize)> = project_source_paths
        .into_iter()
        .map(|path| {
            let file = File::open(path.clone()).unwrap();
            let buf_reader = BufReader::new(file);
            (path, metrics::loc(buf_reader))
        })
        .collect();
    path_and_loc.sort_by(|a, b| b.1.cmp(&a.1));
    path_and_loc
}

fn create_table(path_and_loc: Vec<(PathBuf, usize)>) -> Table {
    let initial_table = init_table_with_header();
    path_and_loc.into_iter().fold(
        initial_table,
        |mut result_table,
         path_and_count| {
            let path = &path_and_count.0.display().to_string();
            let count = &path_and_count.1.to_string();
            result_table.add_row(Row::new(vec![Cell::new(path), Cell::new(count)]));
            result_table
        },
    )
}

fn init_table_with_header() -> Table {
    let mut initial_table = Table::new();
    initial_table.add_row(Row::new(vec![Cell::new("Path"), Cell::new("LOC")]));
    initial_table
}

fn determine_realtive_source_paths() -> Vec<PathBuf> {
    let pwd = std::env::current_dir().unwrap();
    let gitignore_path = pwd.join(".gitignore");
    let source_files = gitignore::File::new(&gitignore_path).expect("no .gitignore found in current directory");
    source_files
        .included_files()
        .unwrap()
        .into_iter()
        .filter(|path| !path.is_dir())
        .map(|path| PathBuf::from(path.strip_prefix(&pwd).unwrap()))
        .collect()
}
