# posfz

При большом объёме обрабатываемых записей паникует на "АЗИТРОМИЦИН-":

thread 'main' panicked at 'byte index 21 is not a char boundary; it is inside 'Н' (bytes 20..22) of `АЗИТРОМИЦИН-`', /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library/core/src/str/mod.rs:678:13


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
