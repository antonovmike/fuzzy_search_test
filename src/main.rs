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
        if index > 60 && index < 190 {
            let sliced = line;
            lines.push(sliced.to_string())
        }
    }

    let mut products: Vec<String> = vec![];

    let mut engine: SimSearch<u32> = SimSearch::new();
    let mut search_id = 0;
    let mut catalog: Vec<(u32, String)> = vec![];

    #[allow(unused_assignments)]
    for i in lines.clone() {
        // Exclude useless data
        let mut shorter_string = "";
        if i.len() > 398 {
            shorter_string = &i[398..]
        }
        let parts = shorter_string.split("N");
        let mut index = 0;
        for part in parts {
            // Remove 1st and last chars
            let mut chars = part.chars();
            chars.next();
            chars.next_back();
            let raw_str = chars.as_str();
            // Exclude useless str
            let mut pure_str = ("", "");
            if raw_str.len() > 2 {
                // Remove last 2 chars
                pure_str = raw_str.split_at(raw_str.len() - 2)
            } else {
                continue;
            }
            if index == 1 {
                // println!("{}\t{}", search_id, pure_str.0);
                products.push(pure_str.0.to_string());
                engine.insert(search_id, pure_str.0);
                catalog.push((search_id, pure_str.0.to_string()));
                search_id += 1;
            }
            if index == 2 {
                // println!("{}\t{}", search_id, pure_str.0);
                products.push(pure_str.0.to_string());
                engine.insert(search_id, pure_str.0);
                catalog.push((search_id, pure_str.0.to_string()));
                search_id += 1;
            }
            if index > 2 {
                continue;
            }
            index += 1;
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
    }
}
