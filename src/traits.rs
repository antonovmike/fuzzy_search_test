use simsearch::{SearchOptions, SimSearch};
use strsim::{
    damerau_levenshtein, jaro, jaro_winkler, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance,
};

pub struct SimSearchEngine {
    engine: SimSearch<usize>,
}

impl SimSearchEngine {
    pub fn new() -> Self {
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

pub struct StrSearchEngine {
    catalog: Vec<(usize, String)>,
}

impl StrSearchEngine {
    pub fn new() -> Self {
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
            // .map(|(i, (_u, s))| (i, damerau_levenshtein(input, s) as f64))
            .map(|(i, (_u, s))| (i, jaro(input, s) as f64))
            // .map(|(i, (_u, s))| (i, osa_distance(input, s) as f64))
            // .map(|(i, d)| (i, d.abs()))
            // .filter(|(_i, d)| d.is_normal())
            // .filter(|(_i, d)| [std::num::FpCategory::Normal].contains(&d.classify()))
            .collect();

        tupvek.sort_by(|(_ia, da), (_ib, db)| db.partial_cmp(da).unwrap());

        tupvek.into_iter().map(|(i, _d)| i).collect()
    }
}

pub trait Search {
    fn name(&self) -> String;
    fn load(&mut self, catalog: Vec<(usize, String)>);
    fn search(&self, input: &str) -> Vec<usize>;
}
