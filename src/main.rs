    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    fn main() {   
        let f = File::open("/home/alarm/git/loc/src/dummy.rs").unwrap();
        let file = BufReader::new(&f);
        count_lines(file) 
    }

    fn count_lines<R: BufRead>(input: R) {
        println!("{}", input.lines().count())
    }
