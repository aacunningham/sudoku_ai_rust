extern crate sudoku_ai;

fn main() {
    let string = "\
        0 4 0 0 6 0 1 2 5 \
        2 6 0 0 4 7 0 0 0 \
        0 0 8 5 3 0 0 0 7 \
        6 0 0 0 5 1 7 3 0 \
        0 7 1 0 0 8 9 0 0 \
        9 0 2 6 0 4 0 0 8 \
        0 5 9 2 0 0 0 0 0 \
        3 1 0 0 8 5 0 0 4 \
        8 0 7 0 9 0 6 0 1";
    let mut puzzle = sudoku_ai::Puzzle::read_from_string(string);
    if let Err(message) = puzzle.solve() {
        println!("{}", message);
    };
    println!("{}", puzzle);
}
