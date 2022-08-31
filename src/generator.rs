use crate::board::Board;
use crate::util::*;
use rand::prelude::*;

fn random_set(rng: &mut rand::prelude::ThreadRng) -> Vec<u8> {
    let mut vec: Vec<u8> = (1..10).collect();
    vec.shuffle(rng);
    vec
}

fn num_used_in_row(n: u8, board: &Board, row: usize) -> bool {
    for i in 0..9 {
        if board[cell_index(row, i)] == Some(n) {
            return true;
        }
    }
    false
}

fn num_used_in_column(n: u8, board: &Board, column: usize) -> bool {
    for i in 0..9 {
        if board[cell_index(i, column)] == Some(n) {
            return true;
        }
    }
    false
}

fn num_used_in_house(n: u8, board: &Board, house: usize) -> bool {
    let (row_start, column_start) = starting_row_and_column_from_house(house);
    for row in row_start..row_start + 3 {
        for column in column_start..column_start + 3 {
            if board[cell_index(row, column)] == Some(n) {
                return true;
            }
        }
    }
    false
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
            if board[cell_index(row, column)] == None {
                for n in 1..10 {
                    if is_valid_location_for(n, board, row, column) {
                        let mut cloned_board = board.clone();
                        cloned_board[cell_index(row, column)] = Some(n);
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

pub fn generate_board(rng: &mut ThreadRng, board: &mut Board) -> bool {
    for row in 0..9 {
        for column in 0..9 {
            if board[cell_index(row, column)] == None {
                let number_list = random_set(rng);
                for n in number_list {
                    if is_valid_location_for(n, &board, row, column) {
                        board[cell_index(row, column)] = Some(n);
                        if !board.has_empty() {
                            return true;
                        } else {
                            if generate_board(rng, board) {
                                return true;
                            }
                        }
                    }
                }
                board[cell_index(row, column)] = None;
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

pub fn remove_numbers(rng: &mut ThreadRng, board: &Board) -> Board {
    let mut puzzle = board.clone();
    let all_cells = shuffle_all_cells(rng);
    let mut hints = 81;
    for (row, column) in all_cells {
        if hints == 80 {
            break;
        }
        let value = puzzle[cell_index(row, column)];
        puzzle[cell_index(row, column)] = None;
        hints -= 1;
        if solve_puzzle(&puzzle.clone(), 0) != 1 {
            println!("Ran into no solution");
            puzzle[cell_index(row, column)] = value;
            hints += 1;
        }
    }
    puzzle
}
