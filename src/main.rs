use simsearch::SimSearch;
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

    let mut engine: SimSearch<u32> = SimSearch::new();
    let mut search_id = 0;
    let mut catalog: Vec<(u32, String)> = vec![];

    for i in lines.clone() {
        let shorter_string = &i[398..];
        let parts = shorter_string.split("N");
        let mut index = 0;
        for part in parts {
            // Remove 1st n 2nd chars
            let mut chars = part.chars();
            chars.next();
            chars.next_back();
            let raw_str = chars.as_str();
            // remove last 2 chars
            let pure_str = raw_str.split_at(raw_str.len() - 2);
            if index == 1 {
                println!("{}\t{}", search_id, pure_str.0);
                products.push(pure_str.0.to_string());
                engine.insert(search_id, pure_str.0);
                catalog.push((search_id, pure_str.0.to_string()));
            }
            if index == 2 {
                println!("{}\t{}", search_id, pure_str.0);
                products.push(pure_str.0.to_string());
                engine.insert(search_id, pure_str.0);
                catalog.push((search_id, pure_str.0.to_string()));
            }

            index += 1;
            search_id += 1
        }
    }

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string().to_uppercase(); // remove leading/trailing whitespaces

        let results: Vec<u32> = engine.search(&input);

        if input == "ВЫХОД" {
            break;
        }

        #[allow(unused_assignments)]
        for i in results.clone() {
            let vec_index = catalog.iter().position(|r| r.0 == i).unwrap();
            let mut final_string = "".to_string();
            if vec_index % 2 == 1 {
                final_string = format!("{}, {}", catalog[vec_index - 1].1, catalog[vec_index].1)
            } else {
                final_string = format!("{}, {}", catalog[vec_index].1, catalog[vec_index + 1].1)
            }
            println!("Найдено: {}", final_string);
        }
        // if products.contains(&input) {
        //     println!("Найдено")
        // } else {
        //     println!("Не найдено")
        // }
    }
}
