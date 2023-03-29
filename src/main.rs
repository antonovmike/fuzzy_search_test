use simsearch::{SearchOptions, SimSearch};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

fn main() {
    let text_file = "utf8_dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();

    // let mut engine: SimSearch<u32> = SimSearch::new();
    let mut engine: SimSearch<u32> = SimSearch::new_with(SearchOptions::new().threshold(0.9));
    let mut search_id = 0;

    let mut catalog: Vec<(u32, String)> = vec![];

    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.starts_with("INSERT"))
        .map(|l| l[398..].to_string())
        .map(|l| {
            let name = l.split("N'").nth(1).unwrap();
            name[0..name.len() - 3].to_owned()
        })
        .for_each(|name| {
            engine.insert(search_id, &name);
            catalog.push((search_id, name.to_string()));

            search_id += 1;
        });
    // .take(500);

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

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

#[test]
fn mistape() {
    let text_file = "utf8_dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();
    let mut engine: SimSearch<u32> = SimSearch::new_with(SearchOptions::new().threshold(0.9));
    let mut search_id = 0;
    let mut catalog: Vec<(u32, String)> = vec![];

    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.starts_with("INSERT"))
        .map(|l| l[398..].to_string())
        .map(|l| {
            let name = l.split("N'").nth(1).unwrap();
            name[0..name.len() - 3].to_owned()
        })
        .for_each(|name| {
            engine.insert(search_id, &name);
            catalog.push((search_id, name.to_string()));

            search_id += 1;
        });

    let input = "верблжй";
    let results: Vec<u32> = engine.search(&input);
    let total = results.len();
    assert_eq!(13, total)
}
