use simsearch::SimSearch;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

fn main() {
    let text_file = "utf8_dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();

    let it = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .filter(|l| l.starts_with("INSERT"))
        .map(|l| l[398..].to_owned());
    // .take(500);

    let mut engine: SimSearch<u32> = SimSearch::new();
    let mut search_id = 0;

    let mut catalog: Vec<(u32, String)> = vec![];

    for i in it {
        let parts = i.split("N'");
        for (i, part) in parts.enumerate() {
            if i == 1 {
                // remove last 3 chars
                let name = part[0..part.len() - 3].to_string();
                println!("{}\t{}", search_id, name);

                engine.insert(search_id, &name);
                catalog.push((search_id, name));

                search_id += 1;
            }
            if i > 2 {
                continue;
            }
        }
    }

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // remove leading/trailing whitespaces
        input = input.trim().to_string(); // .to_uppercase();

        let results: Vec<u32> = engine.search(&input);

        let total = results.len();
        if total == 0 {
            println!("Нет совпадений");
            continue;
        }

        for index in results {
            println!("{}, {:?}", index, catalog[index as usize])
        }
        println!("всего: {}", total);
    }
}
