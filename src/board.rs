use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Board {
    cells: [Option<u8>; 81],
}

impl Board {
    pub fn new() -> Board {
        Board { cells: [None; 81] }
    }

    pub fn has_empty(&self) -> bool {
        self.cells
            .iter()
            .fold(false, |acc, cell| acc || cell.is_none())
    }

    pub fn to_soduko(&self) -> String {
        self.cells
            .map(|cell| match cell {
                Some(value) => value.to_string(),
                None => String::from("0"),
            })
            .join("")
    }
}

impl Index<usize> for Board {
    type Output = Option<u8>;
    fn index<'a>(&'a self, i: usize) -> &'a Option<u8> {
        &self.cells[i]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Option<u8> {
        &mut self.cells[i]
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            let row = i / 9;
            let column = i % 9;
            match cell {
                Some(value) => write!(f, "{} ", value).unwrap(),
                None => write!(f, ". ").unwrap(),
            };
            if column == 2 || column == 5 {
                write!(f, "| ").unwrap();
            }
            if column == 8 {
                writeln!(f, "").unwrap();
                if row == 2 || row == 5 {
                    writeln!(f, "------+-------+------").unwrap();
                }
            }
        }
        Ok(())
    }
}
