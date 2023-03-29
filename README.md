# posfz


Слова для демо версии поика:

ЭЛЬКАР Р-Р 30% 25МЛ

ПИК-ФАРМА ПРО, РОССИЯ

ВИТРУМ ТАБ №130

ЮНИФАРМ, США

АЛФАГИН КАПС №20

ХЕРБИОН, ПАКИСТАН


механизмы нечеткого поиска / fuzzy search

https://lib.rs/crates/simsearch

https://crates.io/crates/strsim

commandline interface https://lib.rs/crates/rustyline

По умолчанию точность поика simsearch 0,8. Запрос ПОЯС ИЗ ВЕРБЛЮЖЬЕЙ ШЕРСТИ ТОНУС Р. 48 выдаст 884 совпадений
```Rust
let mut engine: SimSearch<u32> = SimSearch::new(); = 884
```
Опция с ограничением 0.9 выдаст 464
```Rust
let mut engine: SimSearch<u32> = SimSearch::new_with(SearchOptions::new().threshold(0.9)); = 464
```
