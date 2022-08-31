pub fn house_from_row_and_column(row: usize, column: usize) -> usize {
    (row / 3) * 3 + column / 3
}

pub fn starting_row_and_column_from_house(house: usize) -> (usize, usize) {
    (house / 3 * 3, house % 3 * 3)
}
