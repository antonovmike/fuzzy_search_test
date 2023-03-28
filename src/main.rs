use std::fs::File;
use std::io::BufReader;

fn main() {
    let text_file = "dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();
    let reader = BufReader::new(file);

    let s = utf16_reader::read_to_string(reader);

    let mut lines: Vec<String> = vec![];

    for (index, line) in s.lines().enumerate() {
        if index > 60 && index < 64 {
            let sliced = line;
            lines.push(sliced.to_string())
        }
    }

    for i in lines {
        let shorter_string = &i[398..];
        let parts = shorter_string.split("N");
        let mut index = 0;
        for part in parts {
            if index == 1 || index == 2 {
                let mut chars = part.chars();
                chars.next();
                chars.next_back();
                let raw_str = chars.as_str();
                let pure_str = raw_str.split_at(raw_str.len() - 2);
                println!("{}\t{}", index, pure_str.0)
            }
            index += 1
        }
    }
}
