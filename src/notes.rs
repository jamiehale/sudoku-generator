use crate::util::*;

#[derive(Clone, Copy)]
struct PencilMarks {
    marks: [bool; 9],
}

impl PencilMarks {
    fn new() -> PencilMarks {
        PencilMarks { marks: [false; 9] }
    }

    fn from(marks: Vec<u8>) -> PencilMarks {
        let mut new_marks = [false; 9];
        for mark in marks {
            new_marks[mark as usize - 1] = true;
        }
        PencilMarks { marks: new_marks }
    }

    fn with(&self, marks: Vec<u8>) -> PencilMarks {
        let mut new_marks = self.marks;
        for mark in marks {
            new_marks[mark as usize - 1] = true;
        }
        PencilMarks { marks: new_marks }
    }

    fn without(&self, marks: Vec<u8>) -> PencilMarks {
        let mut new_marks = self.marks;
        for mark in marks {
            new_marks[mark as usize - 1] = false;
        }
        PencilMarks { marks: new_marks }
    }

    fn set(&mut self, mark: u8) {
        self.marks[mark as usize - 1] = true;
    }

    fn clear(&mut self, mark: u8) {
        self.marks[mark as usize - 1] = true;
    }

    fn is_set(&self, mark: u8) -> bool {
        self.marks[mark as usize - 1]
    }

    fn is_clear(&self, mark: u8) -> bool {
        !self.marks[mark as usize - 1]
    }
}

#[derive(Clone, Copy)]
enum NoteCell {
    Partial(PencilMarks),
    Assigned(u8),
    Empty,
}

pub struct Notes {
    cells: [NoteCell; 81],
}

fn cell_index(row: usize, column: usize) -> usize { row * 9 + column }

impl Notes {
    pub fn new() -> Notes {
        Notes { cells: [NoteCell::Empty; 81] }
    }

    fn mark_at(&mut self, value: u8, row: usize, column: usize) {
        self.cells[cell_index(row, column)] = match self.cells[cell_index(row, column)] {
            NoteCell::Empty => {
                NoteCell::Partial(PencilMarks::from(vec![value]))
            },
            NoteCell::Partial(marks) => {            
                NoteCell::Partial(marks.with(vec![value]))
            }
            NoteCell::Assigned(value) => NoteCell::Assigned(value),
        }
    }

    fn clear_at(&mut self, value: u8, row: usize, column: usize) {
        self.cells[cell_index(row, column)] = match self.cells[cell_index(row, column)] {
            NoteCell::Empty => NoteCell::Empty,
            NoteCell::Partial(marks) => NoteCell::Partial(marks.without(vec![value])),
            NoteCell::Assigned(value) => NoteCell::Assigned(value),
        }
    }

    fn clear_row(&mut self, value: u8, row: usize) {
        for column in 0..9 {
            self.cells[cell_index(row, column)] = match self.cells[cell_index(row, column)] {
                NoteCell::Empty => NoteCell::Empty,
                NoteCell::Partial(marks) => NoteCell::Partial(marks.without(vec![value])),
                NoteCell::Assigned(value) => NoteCell::Assigned(value),
            }
        }
    }

    fn clear_column(&mut self, value: u8, column: usize) {
        for row in 0..9 {
            self.cells[cell_index(row, column)] = match self.cells[cell_index(row, column)] {
                NoteCell::Empty => NoteCell::Empty,
                NoteCell::Partial(marks) => NoteCell::Partial(marks.without(vec![value])),
                NoteCell::Assigned(value) => NoteCell::Assigned(value),
            }
        }
    }

    fn clear_house(&mut self, value: u8, house: usize) {
        let (starting_row, starting_column) = starting_row_and_column_from_house(house);
        for row in starting_row..(starting_row + 3) {
            for column in starting_column..(starting_column + 3) {
                self.cells[cell_index(row, column)] = match self.cells[cell_index(row, column)] {
                    NoteCell::Empty => NoteCell::Empty,
                    NoteCell::Partial(marks) => NoteCell::Partial(marks.without(vec![value])),
                    NoteCell::Assigned(value) => NoteCell::Assigned(value),
                }
            }
        }
    }

    fn assign_at(&mut self, value: u8, row: usize, column: usize) {
        self.cells[cell_index(row, column)] = NoteCell::Assigned(value);
    }
}
