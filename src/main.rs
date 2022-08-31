extern crate rand;

mod board;
mod generator;
mod notes;
mod util;

use board::Board;
use generator::{generate_board, remove_numbers};
use notes::Notes;

fn generate_notes(puzzle: &Board) -> Notes {
    let mut notes = Notes::new();
    for i in 0..81 {
        let row = i / 9;
        let column = i % 9;
        match puzzle[i] {
            Some(value) => notes.assign_at(value, row, column),
            None => {}
        }
    }
    notes
}

fn main() {
    let mut board = Board::new();
    let mut rng = rand::thread_rng();
    generate_board(&mut rng, &mut board);
    println!("{:?}", board);
    let puzzle = remove_numbers(&mut rng, &mut board);
    println!("{:?}", puzzle);
    println!("{}", puzzle.to_soduko());
    let notes = generate_notes(&puzzle);
    println!("{:?}", notes);
}
