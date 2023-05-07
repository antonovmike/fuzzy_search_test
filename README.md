# posfz

Fuzzy search test

A real case is used as an example. The names of all goods have been replaced.

Search words and phrases can be taken from the file goods.txt

1. SimSearch
https://lib.rs/crates/simsearch

The default search accuracy of simsearch is 0.8. To increase the accuracy, you should set the value to 0.9.

2. StrSim
https://crates.io/crates/strsim

When switching from usize metric to f64 metric, you have to change the order of comparison:
```Rust
db.partial_cmp(da) // для f64
da.partial_cmp(db) // для usize
````

3. Tentivy
https://lib.rs/crates/tantivy

https://tantivy-search.github.io/examples/basic_search.html

score_with, case_sensitive, case_insensitive, best match

4. Sublime Fuzzy
https://lib.rs/crates/sublime_fuzzy

5. rust-fuzzy-search
https://crates.io/crates/rust-fuzzy-search

commandline interface https://lib.rs/crates/rustyline
