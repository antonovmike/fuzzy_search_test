# posfz


механизмы нечеткого поиска / fuzzy search

1. SimSearch
https://lib.rs/crates/simsearch

По умолчанию точность поика simsearch 0,8. Для повышения точности надо установить значение 0.9.

2. StrSim
https://crates.io/crates/strsim

При переключении с usize метрики на f64 метрику надо менять порядок сравнения:
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

