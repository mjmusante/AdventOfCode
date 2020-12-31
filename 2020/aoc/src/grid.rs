use std::ops::Range;
use crate::utils::lines;
use std::collections::HashSet;
use std::path::Path;

pub struct Grid2D {
    grid: HashSet::<(i64, i64)>,
    min_row: i64,
    min_col: i64,
    max_row: i64,
    max_col: i64,
}

impl Grid2D {
    pub fn new_from_file(filename: impl AsRef<Path>) -> Grid2D {
        let lines = lines(filename);

        Grid2D::new_from_vec(&lines)
    }

    pub fn new_from_vec(lines: &Vec<String>) -> Grid2D {
        let mut result = Grid2D { grid: HashSet::new(), min_row: 0, min_col: 0, max_row: 0, max_col: 0 };

        for l in lines {
            let mut col = 0;
            for ch in l.chars() {
                match ch {
                    '#' => { result.grid.insert((result.max_row, col)); },
                    '.' => {},
                    x => { panic!(format!("Invalid char '{}' in input", x)); },
                };
                col += 1;
            }
            if col > result.max_col {
                result.max_col = col;
            }
            result.max_row += 1;
        }

        result
    }

    pub fn rows(&self) -> Range<i64> {
        self.min_row..self.max_row
    }

    pub fn row_count(&self) -> i64 {
        self.max_row - self.min_row
    }

    pub fn col_count(&self) -> i64 {
        self.max_col - self.min_col
    }

    pub fn contains(&self, row: i64, col: i64) -> bool {
        self.grid.contains(&(row, col))
    }
}
