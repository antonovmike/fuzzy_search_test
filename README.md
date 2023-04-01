# posfz


механизмы нечеткого поиска / fuzzy search

1. SimSearch
https://lib.rs/crates/simsearch

По умолчанию точность поика simsearch 0,8. Запрос ПОЯС ИЗ ВЕРБЛЮЖЬЕЙ ШЕРСТИ ТОНУС Р. 48 выдаст 884 совпадений
```Rust
let mut engine: SimSearch<u32> = SimSearch::new(); = 884
```
Опция с ограничением 0.9 выдаст 464
```Rust
let mut engine: SimSearch<u32> = SimSearch::new_with(SearchOptions::new().threshold(0.9)); = 464
```

2. StrSim
https://crates.io/crates/strsim

При переключении с usize метрики на f64 метрику надо менять порядок сравнения:
```Rust
db.partial_cmp(da) // для f64
da.partial_cmp(db) // для usize
````

3. Tentivy
https://lib.rs/crates/tantivy

4. Sublime Fuzzy
https://lib.rs/crates/sublime_fuzzy

commandline interface https://lib.rs/crates/rustyline

