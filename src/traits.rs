#[allow(unused)]
#[macro_use]
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{schema::*, Index, ReloadPolicy};
use tempfile::TempDir;

#[allow(unused)]
use rust_fuzzy_search::{fuzzy_compare, fuzzy_search, fuzzy_search_sorted, fuzzy_search_threshold};
use simsearch::{SearchOptions, SimSearch};
#[allow(unused)]
use strsim::{
    damerau_levenshtein, jaro, jaro_winkler, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance,
};

pub trait Search {
    fn name(&self) -> String;
    fn load(&mut self, catalog: Vec<(usize, String)>);
    fn search(&self, input: &str) -> Vec<usize>;
}

// -----------------------------

pub struct TantivySearch {
    catalog: Vec<(usize, String)>,
}

impl TantivySearch {
    pub fn new() -> Self {
        TantivySearch {
            catalog: Vec::new(),
        }
    }
}

impl Search for TantivySearch {
    fn name(&self) -> String {
        return "TantivySearch".into();
    }

    fn load(&mut self, mut catalog: Vec<(usize, String)>) {
        self.catalog.append(&mut catalog);
    }

    fn search(&self, input: &str) -> Result<(), Vec<usize>> {
        let index_path = TempDir::new()?;
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("body", TEXT | STORED);
        let schema = schema_builder.build();
        let index = Index::create_in_dir(&index_path, schema.clone())?;
        let mut index_writer = index.writer(50_000_000)?;
        let body = schema.get_field("body").unwrap();

        Ok(())
    }
}
#[macro_use]
fn tan() -> tantivy::Result<()> {
    let index_path = TempDir::new()?;

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("body", TEXT | STORED);

    let schema = schema_builder.build();

    let index = Index::create_in_dir(&index_path, schema.clone())?;

    let mut index_writer = index.writer(50_000_000)?;

    let body = schema.get_field("body").unwrap();

    let database = [
        "ПОЯС ДЛЯ СПИНЫ И ПОЗВОНОЧНИКА Р. 54",
        "ПОЯС ДЛЯ ПОХУДЕНИЯ ТОНУС Р. 48-50",
        "ПОЯС ДЛЯ ПОДДЕРЖКИ ЖИВОТА ТОНУС Р. 54",
        "ПОЯС ДЛЯ ПОХУДЕНИЯ ТОНУС",
        "ПОЯС ЭЛАСТИЧНЫЙ Р. 48",
        "ПОЯС ПОСЛЕОПЕРАЦИОНЫЙ",
        "ПОЯС ДЛЯ ПОДДЕРЖКИ ЖИВОТА ТОНУС Р. 54",
    ];
    #[allow(unused)]
    #[macro_use]
    for i in database {
        index_writer.add_document(doc!(
            body => i
        ));
        index_writer.commit()?;
    }

    index_writer.commit()?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![body]);

    let query = query_parser.parse_query("похудения")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let the_answer = schema.to_json(&retrieved_doc);
        println!("{} {}", score, the_answer);
    }

    Ok(())
}

// -----------------------------

pub struct RustFuzzySearch {
    catalog: Vec<(usize, String)>,
}

impl RustFuzzySearch {
    pub fn new() -> Self {
        RustFuzzySearch {
            catalog: Vec::new(),
        }
    }
}

impl Search for RustFuzzySearch {
    fn name(&self) -> String {
        return "RustFuzzySearch".into();
    }

    fn load(&mut self, mut catalog: Vec<(usize, String)>) {
        self.catalog.append(&mut catalog);
    }

    fn search(&self, input: &str) -> Vec<usize> {
        let mut tupvek: Vec<(usize, f64)> = self
            .catalog
            .iter()
            .enumerate()
            .map(|(i, (_u, s))| {
                (
                    i,
                    fuzzy_compare(&input.to_lowercase(), &s.to_lowercase()) as f64,
                )
            })
            .collect();

        tupvek.sort_by(|(_ia, da), (_ib, db)| db.partial_cmp(da).unwrap());

        tupvek.into_iter().map(|(i, _d)| i).collect()
    }
}

// -----------------------------

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

// -----------------------------

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
            .map(|(i, (_u, s))| (i, jaro(&input.to_lowercase(), &s.to_lowercase()) as f64))
            // .map(|(i, (_u, s))| (i, osa_distance(input, s) as f64))
            // .map(|(i, d)| (i, d.abs()))
            // .filter(|(_i, d)| d.is_normal())
            // .filter(|(_i, d)| [std::num::FpCategory::Normal].contains(&d.classify()))
            .collect();

        tupvek.sort_by(|(_ia, da), (_ib, db)| db.partial_cmp(da).unwrap());

        tupvek.into_iter().map(|(i, _d)| i).collect()
    }
}
