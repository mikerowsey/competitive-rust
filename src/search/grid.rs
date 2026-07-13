pub const CARDINAL_DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub const KNIGHT_DIRS: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

#[inline]
pub fn in_bounds(row: i32, column: i32, rows: i32, columns: i32) -> bool {
    row >= 0 && row < rows && column >= 0 && column < columns
}
