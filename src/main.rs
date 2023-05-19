mod sudoku_board;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut board = sudoku_board::Board::new();
    if board.load(&args.path) {
        println!("Here's the puzzle we need to solve:");
        board.print();
        println!("");

        if board.solve() {
            println!("Here is the solution:");
        } else {
            println!("We couldn't solve it. Here's where we got stuck:");
        }
        board.print();
        println!("");
    }
}
