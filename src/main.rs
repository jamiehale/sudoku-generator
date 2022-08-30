extern crate rand;

use itertools::Itertools;
use rand::prelude::*;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
struct Row {
    cells: [Option<u8>; 9],
}

impl Row {
    fn new() -> Row {
        Row { cells: [None; 9] }
    }

    fn has_empty(&self) -> bool {
        self.cells
            .iter()
            .fold(false, |acc, value| acc || value.is_none())
    }

    fn to_soduko(&self) -> String {
        self.cells
            .map(|cell| match cell {
                Some(x) => x.to_string(),
                None => String::from("0"),
            })
            .join("")
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    match cell {
                        Some(x) => x.to_string() + if i == 2 || i == 5 { " |" } else { "" },
                        None => String::from(".") + if i == 2 || i == 5 { " |" } else { "" },
                    }
                })
                .join(" ")
        )
    }
}

impl Index<usize> for Row {
    type Output = Option<u8>;
    fn index<'a>(&'a self, i: usize) -> &'a Option<u8> {
        &self.cells[i]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Option<u8> {
        &mut self.cells[i]
    }
}

#[derive(Clone, Copy)]
struct Board {
    rows: [Row; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            rows: [Row::new(); 9],
        }
    }

    fn has_empty(&self) -> bool {
        self.rows
            .iter()
            .fold(false, |acc, row| acc || row.has_empty())
    }

    fn to_soduko(&self) -> String {
        self.rows.map(|row| row.to_soduko()).join("")
    }
}

impl Index<usize> for Board {
    type Output = Row;
    fn index<'a>(&'a self, i: usize) -> &'a Row {
        &self.rows[i]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Row {
        &mut self.rows[i]
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.rows.iter().enumerate() {
            writeln!(f, "{:?}", row).unwrap();
            if i == 2 || i == 5 {
                writeln!(f, "------+-------+------").unwrap();
            }
        }
        Ok(())
    }
}

fn random_set(rng: &mut rand::prelude::ThreadRng) -> Vec<u8> {
    let mut vec: Vec<u8> = (1..10).collect();
    vec.shuffle(rng);
    vec
}

fn num_used_in_row(n: u8, board: &Board, row: usize) -> bool {
    for i in 0..9 {
        if board[row][i] == Some(n) {
            return true;
        }
    }
    false
}

fn num_used_in_column(n: u8, board: &Board, column: usize) -> bool {
    for i in 0..9 {
        if board[i][column] == Some(n) {
            return true;
        }
    }
    false
}

fn num_used_in_house(n: u8, board: &Board, house: usize) -> bool {
    let (row_start, column_start) = starting_row_and_column_from_house(house);
    for row in row_start..row_start + 3 {
        for column in column_start..column_start + 3 {
            if board[row][column] == Some(n) {
                return true;
            }
        }
    }
    false
}

fn house_from_row_and_column(row: usize, column: usize) -> usize {
    (row / 3) * 3 + column / 3
}

fn starting_row_and_column_from_house(house: usize) -> (usize, usize) {
    (house / 3 * 3, house % 3 * 3)
}

fn is_valid_location_for(n: u8, board: &Board, row: usize, column: usize) -> bool {
    if num_used_in_row(n, board, row) {
        return false;
    }
    if num_used_in_column(n, board, column) {
        return false;
    }
    if num_used_in_house(n, board, house_from_row_and_column(row, column)) {
        return false;
    }
    true
}

fn solve_puzzle(board: &Board, solution_count: u32) -> u32 {
    for row in 0..9 {
        for column in 0..9 {
            if board[row][column] == None {
                for n in 1..10 {
                    if is_valid_location_for(n, board, row, column) {
                        let mut cloned_board = board.clone();
                        cloned_board[row][column] = Some(n);
                        if !cloned_board.has_empty() {
                            return solution_count + 1;
                        } else {
                            return solve_puzzle(&cloned_board, solution_count);
                        }
                    }
                }
                return solution_count;
            }
        }
    }
    solution_count
}

fn generate_board(rng: &mut ThreadRng, board: &mut Board) -> bool {
    for row in 0..9 {
        for column in 0..9 {
            if board[row][column] == None {
                let number_list = random_set(rng);
                for n in number_list {
                    if is_valid_location_for(n, &board, row, column) {
                        board[row][column] = Some(n);
                        if !board.has_empty() {
                            return true;
                        } else {
                            if generate_board(rng, board) {
                                return true;
                            }
                        }
                    }
                }
                board[row][column] = None;
                return false;
            }
        }
    }
    false
}

fn shuffle_all_cells(rng: &mut ThreadRng) -> Vec<(usize, usize)> {
    let mut all_cells: Vec<(usize, usize)> = vec![];
    for row in 0..9 {
        for column in 0..9 {
            all_cells.push((row, column));
        }
    }
    all_cells.shuffle(rng);
    all_cells
}

fn remove_numbers(rng: &mut ThreadRng, board: &mut Board) {
    let all_cells = shuffle_all_cells(rng);
    let mut rounds = 0;
    let mut hints = 81;
    for (row, column) in all_cells {
        if hints == 17 {
            break;
        }
        let value = board[row][column];
        board[row][column] = None;
        hints -= 1;
        if solve_puzzle(&board.clone(), 0) != 1 {
            println!("Ran into no solution");
            board[row][column] = value;
            rounds += 1;
            hints += 1;
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut rng = rand::thread_rng();
    generate_board(&mut rng, &mut board);
    println!("{:?}", board);
    remove_numbers(&mut rng, &mut board);
    println!("{:?}", board);
    println!("{}", board.to_soduko());
}
