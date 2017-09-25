extern crate glob;

mod loc;

use std::fs::File;
use std::io::BufReader;
use glob::glob;

fn main() {
    let lines = glob(&"./**/*.rs").unwrap().into_iter()
        .map(|path_result| path_result.unwrap())
        .filter(|path| !path.is_dir())
        .map(|path| {
            let file = File::open(path.clone()).unwrap();
            let buf_reader = BufReader::new(file);
            loc::count_lines_of_code(buf_reader)
        });

    for line in lines {
        println!("{:?}", line);
    }
}
