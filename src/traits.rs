use tantivy::collector::TopDocs;
// use tantivy::{doc, Document};
use tantivy::Document;
use tantivy::query::QueryParser;
// use tantivy::{Index, ReloadPolicy, DocId, DocAddress};
use tantivy::{Index, ReloadPolicy};
// use std::collections::HashMap;

#[allow(unused)]
use rust_fuzzy_search::{fuzzy_compare, fuzzy_search, fuzzy_search_sorted, fuzzy_search_threshold};
use simsearch::{SearchOptions, SimSearch};
#[allow(unused)]
use strsim::{
    damerau_levenshtein, jaro, jaro_winkler, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance,
};
use tantivy::schema::{Schema, STORED, TEXT, Value};

pub trait Search {
    fn name(&self) -> String;
    fn load(&mut self, catalog: Vec<(usize, String)>);
    fn search(&self, input: &str) -> Vec<usize>;
}

// -----------------------------

pub struct TantivySearch {
    // schema: Schema,
    index: Index,
}

impl TantivySearch {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("body", TEXT);
        schema_builder.add_text_field("id", TEXT | STORED);

        let schema = schema_builder.build();

        let path = "./tantivy";

        let index = Index::create_in_dir(&path, schema).unwrap();

        TantivySearch { index }
    }
}

impl Search for TantivySearch {
    fn name(&self) -> String {
        return "TantivySearch".into();
    }

    fn load(&mut self, catalog: Vec<(usize, String)>) {
        let mut writer = self.index.writer(3_000_000).unwrap();

        let body = self.index.schema().get_field("body").unwrap();
        let id = self.index.schema().get_field("id").unwrap();


        for (index, text) in catalog {
            let mut rec = Document::default();
            rec.add_text(body, text);
            rec.add_u64(id, index as u64);

            writer.add_document(rec).unwrap();
        }

        writer.commit().unwrap();
    }

    fn search(&self, input: &str) -> Vec<usize> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();

        let searcher = reader.searcher();

        let id = self.index.schema().get_field("id").unwrap();
        let body = self.index.schema().get_field("body").unwrap();

        let parser = QueryParser::for_index(&self.index, vec![ body ]);
        let query = parser.parse_query(input).unwrap();

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

        top_docs
            .into_iter()
            .map(|(score, doc_address)| {

// METRICS
                println!("{score}");

                let retrieved_doc = searcher.doc(doc_address).unwrap();

                let index = if let Some(v) = retrieved_doc.get_first(id) {
                    match v {
                        Value::U64(n) => (*n) as usize,
                        _ => usize::MAX
                    }
                } else {
                    usize::MAX
                };

                index
            })
            .filter(|i| *i != usize::MAX)
            .collect()
    }
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

        tupvek
            .iter()
            .take(10)
            .for_each(|(_u, f)| println!("{f}"));

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

        tupvek
            .iter()
            .take(10)
            .for_each(|(_u, f)| println!("{f}"));
        
        tupvek.into_iter().map(|(i, _d)| i).collect()
    }
}
