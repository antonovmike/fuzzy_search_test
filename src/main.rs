use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

mod traits;
use traits::{RustFuzzySearch, Search, SimSearchEngine, StrSearchEngine, TantivySearch};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to open file: {0}")]
    IoError(#[from] std::io::Error),
}

fn main() {
    let catalog = load().unwrap();

    let mut engines: Vec<Box<dyn Search>> = vec![
        Box::new(SimSearchEngine::new()),
        Box::new(StrSearchEngine::new()),
        Box::new(RustFuzzySearch::new()),
        Box::new(TantivySearch::new()),
    ];

    for engine in &mut engines {
        engine.load(catalog.clone());
    }

    loop {
        print!("Enter search phrase: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        for engine in &mut engines {
            println!("--------------------\n\t{}", engine.name());

            let results = engine.search(&input);
            let total = results.len();
            if total == 0 {
                println!("No matches");
                continue;
            }
            results
                .into_iter()
                .take(10)
                .for_each(|i| println!("{i}, {:?}", catalog[i as usize].1));

            println!("Total: {}", total);
        }
    }
}

fn load() -> Result<Vec<(usize, String)>, Error> {
    let text_file = "some_data";
    let file = File::open(text_file)?;

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

    Ok(catalog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_mistape_1() {
        let catalog = load();
        let mut engine = SimSearchEngine::new();
        engine.load(catalog.unwrap().clone());

        let input = "green";
        let results = engine.search(&input);
        let total = results.len();
        assert_eq!(3, total)
    }

    #[test]
    fn total_mistape_2() {
        let catalog = load();
        let mut engine = SimSearchEngine::new();
        engine.load(catalog.unwrap().clone());

        let input = "vanilla";
        let results = engine.search(&input);
        let total = results.len();
        assert_eq!(2, total)
    }

    #[test]
    fn top_10_mistape_1() {
        let input = "blueberry";
        let catalog = load();
        let mut engine = SimSearchEngine::new();

        engine.load(catalog.unwrap().clone());
        engine.search(&input);

        let answervec: Vec<usize> = engine.search(&input).into_iter().take(10).collect();
        let key: Vec<usize> = vec![30, 45, 52];

        assert_eq!(key, answervec)
    }

    #[test]
    fn top_10_mistape_2() {
        let input = "jasmine";
        let catalog = load();
        let mut engine = SimSearchEngine::new();

        engine.load(catalog.unwrap().clone());
        engine.search(&input);

        let answervec: Vec<usize> = engine.search(&input).into_iter().take(10).collect();
        let key: Vec<usize> = vec![66, 97];

        assert_eq!(key, answervec)
    }
}
