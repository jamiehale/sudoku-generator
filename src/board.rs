use itertools::Itertools;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Row {
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
pub struct Board {
    rows: [Row; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            rows: [Row::new(); 9],
        }
    }

    pub fn has_empty(&self) -> bool {
        self.rows
            .iter()
            .fold(false, |acc, row| acc || row.has_empty())
    }

    pub fn to_soduko(&self) -> String {
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
