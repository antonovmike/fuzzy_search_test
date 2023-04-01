use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

mod traits;
use traits::{RustFuzzySearch, Search, SimSearchEngine, StrSearchEngine};

fn main() {
    let catalog = load();

    let mut engines: Vec<Box<dyn Search>> = vec![
        Box::new(SimSearchEngine::new()),
        Box::new(StrSearchEngine::new()),
        Box::new(RustFuzzySearch::new()),
    ];

    for engine in &mut engines {
        engine.load(catalog.clone())
    }

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        for engine in &mut engines {
            println!("\t{}", engine.name());

            let results = engine.search(&input);
            let total = results.len();
            if total == 0 {
                println!("Нет овпадений");
                continue;
            }
            results
                .into_iter()
                .take(10)
                .for_each(|i| println!("{i}, {:?}", catalog[i as usize].1));

            println!("всего: {}", total);
        }
    }
}

fn load() -> Vec<(usize, String)> {
    let text_file = "utf8_dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();

    let mut search_id = 0;

    let mut catalog = vec![];

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
            catalog.push((search_id, name.to_string()));
            search_id += 1;
        });

    catalog
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_mistape_1() {
        let catalog = load();
        let mut engine = SimSearchEngine::new();
        engine.load(catalog.clone());

        let input = "верблжй";
        let results = engine.search(&input);
        let total = results.len();
        assert_eq!(13, total)
    }

    #[test]
    fn total_mistape_2() {
        let catalog = load();
        let mut engine = SimSearchEngine::new();
        engine.load(catalog.clone());

        let input = "эластичн";
        let results = engine.search(&input);
        let total = results.len();
        assert_eq!(222, total)
    }

    #[test]
    fn total_full_prase() {
        let catalog = load();
        let mut engine = SimSearchEngine::new();
        engine.load(catalog.clone());

        let input = "ПОЯС ИЗ ВЕРБЛЮЖЬЕЙ ШЕРСТИ ТОНУС Р. 48";
        let results = engine.search(&input);
        let total = results.len();
        assert_eq!(464, total)
    }

    #[test]
    fn top_10_mistape_1() {
        let input = "верблжй";
        let catalog = load();
        let mut engine = SimSearchEngine::new();

        engine.load(catalog.clone());
        engine.search(&input);

        let answervec: Vec<usize> = engine.search(&input).into_iter().take(10).collect();

        let key: Vec<usize> = vec![
            1943, 4347, 4348, 4363, 4364, 10482, 10483, 10484, 10485, 11237,
        ];

        assert_eq!(key, answervec)
    }

    #[test]
    fn top_10_mistape_2() {
        let input = "эластичн";
        let catalog = load();
        let mut engine = SimSearchEngine::new();

        engine.load(catalog.clone());
        engine.search(&input);

        let answervec: Vec<usize> = engine.search(&input).into_iter().take(10).collect();

        let key: Vec<usize> = vec![1738, 1919, 1921, 1922, 1923, 1924, 1925, 1944, 2236, 2237];

        assert_eq!(key, answervec)
    }
}
