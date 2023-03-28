use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};

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

    let mut products: Vec<String> = vec![];

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string().to_uppercase(); // remove leading/trailing whitespaces

        if input == "ВЫХОД" {
            break;
        }

        for i in lines.clone() {
            let shorter_string = &i[398..];
            let parts = shorter_string.split("N");
            let mut index = 0;
            for part in parts {
                if index == 1 || index == 2 {
                    // Remove 1st n 2nd chars
                    let mut chars = part.chars();
                    chars.next();
                    chars.next_back();
                    let raw_str = chars.as_str();
                    // remove last 2 chars
                    let pure_str = raw_str.split_at(raw_str.len() - 2);
                    // println!("{}\t{}", index, pure_str.0)
                    products.push(pure_str.0.to_string())
                }
                index += 1
            }
        }

        if products.contains(&input) {
            println!("SUCCESS!")
        }
    }
}
