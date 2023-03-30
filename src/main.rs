use simsearch::{SearchOptions, SimSearch};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

struct SimSearchEngine {
    engine: SimSearch<u32>,
}

impl SimSearchEngine {
    fn new() -> Self {
        SimSearchEngine {
            engine: SimSearch::new_with(SearchOptions::new().threshold(0.9)),
        }
    }
}

trait Search {
    fn load(&mut self, catalog: Vec<(u32, String)>);
    fn search(&self, input: &str) -> Vec<u32>;
}

impl Search for SimSearchEngine {
    fn load(&mut self, catalog: Vec<(u32, String)>) {
        catalog
            .iter()
            .for_each(|(i, data)| self.engine.insert(*i, data))
    }
    fn search(&self, input: &str) -> Vec<u32> {
        self.engine.search(input)
    }
}

fn main() {
    let catalog = load();

    let mut engine = SimSearchEngine::new();
    engine.load(catalog.clone());
    // SimSearchEngine::load(catalog);

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
        results
            .into_iter()
            .take(10)
            .for_each(|i| println!("{i}, {:?}", catalog[i as usize].1));

        println!("всего: {}", total);
    }
}

fn load() -> Vec<(u32, String)> {
    let text_file = "utf8_dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();

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
            // engine.insert(search_id, &name);
            catalog.push((search_id, name.to_string()));

            search_id += 1;
        });

    catalog
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn total_mistape_1() {
//         let (_catalog, engine) = load();

//         let input = "верблжй";
//         let results: Vec<u32> = engine.search(&input);
//         let total = results.len();
//         assert_eq!(13, total)
//     }

//     #[test]
//     fn total_mistape_2() {
//         let (_catalog, engine) = load();

//         let input = "эластичн";
//         let results: Vec<u32> = engine.search(&input);
//         let total = results.len();
//         assert_eq!(222, total)
//     }

//     #[test]
//     fn total_full_prase() {
//         let (_catalog, engine) = load();

//         let input = "ПОЯС ИЗ ВЕРБЛЮЖЬЕЙ ШЕРСТИ ТОНУС Р. 48";
//         let results: Vec<u32> = engine.search(&input);
//         let total = results.len();
//         assert_eq!(464, total)
//     }

//     #[test]
//     fn top_10_mistape_1() {
//         let input = "верблжй";
//         let (_catalog, engine) = load();
//         engine.search(&input);

//         let answervec: Vec<u32> = engine.search(&input).into_iter().take(10).collect();

//         let key: Vec<u32> = vec![
//             1943, 4347, 4348, 4363, 4364, 10482, 10483, 10484, 10485, 11237,
//         ];

//         assert_eq!(key, answervec)
//     }

//     #[test]
//     fn top_10_mistape_2() {
//         let input = "эластичн";
//         let (_catalog, engine) = load();
//         engine.search(&input);

//         let answervec: Vec<u32> = engine.search(&input).into_iter().take(10).collect();

//         let key: Vec<u32> = vec![1738, 1919, 1921, 1922, 1923, 1924, 1925, 1944, 2236, 2237];

//         assert_eq!(key, answervec)
//     }
// }
