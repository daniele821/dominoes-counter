#![allow(unused)]

use std::fmt::Display;

const STR_BLOCK: &str = "██";
const COL_CLEAN: &str = "\x1b[0m";
const COL_BACKGROUND: &str = "\x1b[37m";
const COL_RED: &str = "\x1b[31m";
const COL_GREEN: &str = "\x1b[32m";
const COL_YELLOW: &str = "\x1b[33m";
const COL_BLUE: &str = "\x1b[34m";

#[derive(Debug, Clone, PartialEq, Eq)]
enum DominoColor {
    Empty,
    Red,
    Green,
    Yellow,
    Blue,
}

#[derive(Debug)]
struct DominoArea {
    rows: u64,
    cols: u64,
    cells: Vec<DominoColor>,
}

impl DominoArea {
    fn create_empty(rows: u64, cols: u64) -> DominoArea {
        DominoArea {
            rows,
            cols,
            cells: (0..rows * cols).map(|_| DominoColor::Empty).collect(),
        }
    }

    fn row_from_index(&self, index: u64) -> u64 {
        index / self.cols
    }
    fn col_from_index(&self, index: u64) -> u64 {
        index % self.cols
    }
    fn to_index(&self, row: u64, col: u64) -> u64 {
        row * self.cols + col
    }
    fn get_cell_at_index(&self, index: u64) -> &DominoColor {
        self.cells.get(usize::try_from(index).unwrap()).unwrap()
    }
    fn get_cell(&self, row: u64, col: u64) -> &DominoColor {
        self.get_cell_at_index(self.to_index(row, col))
    }
    fn get_cell_at_index_mut(&mut self, index: u64) -> &mut DominoColor {
        self.cells.get_mut(usize::try_from(index).unwrap()).unwrap()
    }
    fn get_cell_mut(&mut self, row: u64, col: u64) -> &mut DominoColor {
        self.get_cell_at_index_mut(self.to_index(row, col))
    }

    fn get_valid_colors(&self, indexes: &[u64]) -> Vec<DominoColor> {
        for index in indexes {
            let cell = self.get_cell_at_index(*index);
            let row = self.row_from_index(*index);
            let col = self.col_from_index(*index);
            let empty = &DominoColor::Empty;
            assert_eq!(cell, empty, "already colored cell ({row},{col})!");
        }
        let cols = i64::try_from(self.cols).unwrap();
        let mut valid_colors = vec![
            DominoColor::Blue,
            DominoColor::Red,
            DominoColor::Green,
            DominoColor::Yellow,
        ];
        for index in indexes {
            let index2 = i64::try_from(*index).unwrap();
            for near_indexes in [index2 - 1, index2 + 1, index2 - cols, index2 + cols] {
                if near_indexes < 0 || near_indexes >= self.cells.len().try_into().unwrap() {
                    continue;
                }
                let cell = self.get_cell_at_index(u64::try_from(near_indexes).unwrap());
                valid_colors.retain(|e| e != cell);
            }
        }
        valid_colors
    }

    fn set_valid_color(&mut self, indexes: &[u64], color: DominoColor) {
        for index in indexes {
            let cell = self.get_cell_at_index_mut(*index);
            *cell = color.clone();
        }
    }
}

impl Display for DominoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DominoColor::Empty => write!(f, "{}{}{}", COL_BACKGROUND, STR_BLOCK, COL_CLEAN),
            DominoColor::Red => write!(f, "{}{}{}", COL_RED, STR_BLOCK, COL_CLEAN),
            DominoColor::Green => write!(f, "{}{}{}", COL_GREEN, STR_BLOCK, COL_CLEAN),
            DominoColor::Yellow => write!(f, "{}{}{}", COL_YELLOW, STR_BLOCK, COL_CLEAN),
            DominoColor::Blue => write!(f, "{}{}{}", COL_BLUE, STR_BLOCK, COL_CLEAN),
        }
    }
}

impl Display for DominoArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.get_cell(row, col).to_string())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut i = DominoArea::create_empty(3, 5);
    println!("{i}");

    let indexes = [
        vec![0, 1, 2],
        vec![3, 8, 13],
        vec![4, 9],
        vec![14],
        vec![5, 6, 11],
        vec![7],
        vec![12],
    ];

    for index in &indexes {
        let colors = i.get_valid_colors(index);
        i.set_valid_color(index, colors[0].clone());
        println!("{i}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_draw() {
        let actual = DominoArea::create_empty(3, 5).to_string();
        let expected = "▀ ▀ ▀ ▀ ▀ \n▀ ▀ ▀ ▀ ▀ \n▀ ▀ ▀ ▀ ▀ \n";
        assert_eq!(actual, expected);
    }
}
