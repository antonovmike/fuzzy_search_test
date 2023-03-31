use simsearch::{SearchOptions, SimSearch};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use strsim::{jaro, jaro_winkler, osa_distance};

struct SimSearchEngine {
    engine: SimSearch<usize>,
}

impl SimSearchEngine {
    fn new() -> Self {
        SimSearchEngine {
            engine: SimSearch::new_with(SearchOptions::new().threshold(0.9)),
        }
    }
}

impl Search for SimSearchEngine {
    fn name(&self) -> String {
        return "SimSearch".into();
    }
    fn load(&mut self, catalog: Vec<(usize, String)>) {
        catalog
            .iter()
            .for_each(|(i, data)| self.engine.insert(*i, data))
    }
    fn search(&self, input: &str) -> Vec<usize> {
        self.engine.search(input)
    }
}

struct StrSearchEngine {
    catalog: Vec<(usize, String)>,
}

impl StrSearchEngine {
    fn new() -> Self {
        StrSearchEngine {
            catalog: Vec::new(),
        }
    }
}

impl Search for StrSearchEngine {
    fn name(&self) -> String {
        return "SrtSimSearch".into();
    }
    fn load(&mut self, mut catalog: Vec<(usize, String)>) {
        self.catalog.append(&mut catalog);
    }
    fn search(&self, input: &str) -> Vec<usize> {
        let mut tupvek: Vec<(usize, f64)> = self
            .catalog
            .iter()
            .enumerate()
            .map(|(i, (_u, s))| (i, jaro(input, s)))
            .map(|(i, d)| (i, d.abs()))
            // .filter(|(_i, d)| d.is_normal())
            // .filter(|(_i, d)| [std::num::FpCategory::Normal].contains(&d.classify()))
            .collect();
        tupvek.sort_by(|(_ia, da), (_ib, db)| db.partial_cmp(da).unwrap());

        tupvek.into_iter().map(|(i, _d)| i).collect()

        // let distances: Vec<usize> = self
        //     .catalog
        //     .iter()
        //     .enumerate()
        //     .map(|(_i, (_u, s))| osa_distance(input, s))
        //     .collect();

        // distances.into_iter().take(10).collect()
    }
}

trait Search {
    fn name(&self) -> String;
    fn load(&mut self, catalog: Vec<(usize, String)>);
    fn search(&self, input: &str) -> Vec<usize>;
}

fn main() {
    let catalog = load();

    // let mut engine = SimSearchEngine::new();
    // let mut engine = StrSearchEngine::new();
    // engine.load(catalog.clone());

    let mut engines: Vec<Box<dyn Search>> = vec![
        Box::new(SimSearchEngine::new()),
        Box::new(StrSearchEngine::new()),
    ];

    loop {
        print!("Текст для поиска: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        // let results = engine.search(&input);

        for engine in &mut engines {
            println!("\t{}", engine.name());
            engine.load(catalog.clone());
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

        // let total = results.len();
        // if total == 0 {
        //     println!("Нет совпадений");
        //     continue;
        // }
        // results
        //     .into_iter()
        //     .take(10)
        //     .for_each(|i| println!("{i}, {:?}", catalog[i as usize].1));

        // println!("всего: {}", total);
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
