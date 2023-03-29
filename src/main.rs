use simsearch::{SearchOptions, SimSearch};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

fn main() {
    let (catalog, engine) = load();

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        let mut results: Vec<u32> = engine.search(&input);

        let total = results.len();
        if total == 0 {
            println!("Нет совпадений");
            continue;
        }
        if total > 10 {
            results.drain(10..);
        }
        for index in results {
            println!("{}, {:?}", index, catalog[index as usize].1)
        }
        println!("всего: {}", total);
    }
}

fn load() -> (Vec<(u32, String)>, SimSearch<u32>) {
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

    (catalog, engine)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_trim() {
    //     assert_eq!("верблжй", trimmer("верблжй ".to_string()));
    // }
    #[test]
    fn mistape_1() {
        let (_catalog, engine) = load();

        let input = "верблжй";
        let results: Vec<u32> = engine.search(&input);
        let total = results.len();
        assert_eq!(13, total)
    }

    #[test]
    fn mistape_2() {
        let (catalog, engine) = load();

        let input = "верблжй";
        engine.search(&input);

        let mut qwe = false;
        for i in catalog.clone() {
            if i.0 == 1943 {
                println!("TEST {:?}", i);
                qwe = true
            }
        }

        assert_eq!(true, qwe)
    }
}
