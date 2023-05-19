# Sudoku
This is a simple command line sudoku solver. I used this project to learn Rust, so it probably isn't done in the most Rust-y way. Also, no documentation was consulted, so this probably isn't the most efficient algorithm.

To use, just pass the board in as a text document.
- There must be 81 valid characters.
- Any character that isn't in '1'-'9' or '*' is ignored.
- Any characters after the 81st valid character are also ignored.

The examples show various possible configurations and puzzle difficulties.
```
cargo run -- sudoku_easy_01.txt
cargo run -- sudoku_easy_02.txt
cargo run -- sudoku_intermediate.txt
cargo run -- sudoku_hard.txt
cargo run -- sudoku_impossible.txt
cargo run -- sudoku_incomplete.txt
```
