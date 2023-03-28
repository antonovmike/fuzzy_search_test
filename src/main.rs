use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let text_file = "example.sql";
    let file = File::open(text_file).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index > 50 && index < 55 {
            let line = line.unwrap();
            println!("{}\n{}", index + 1, line);
        }
    }
}
