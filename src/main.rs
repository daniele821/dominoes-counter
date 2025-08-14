#![allow(unused)]

use std::fmt::Display;

const STR_BLOCK: &str = "  ";
const STR_BLOCK_HIGHLIGHT: &str = "\x1b[1;30m✪ ";
const COL_CLEAN: &str = "\x1b[0m";
const COL_WHITE: &str = "\x1b[1;47m";
const COL_RED: &str = "\x1b[1;41m";
const COL_GREEN: &str = "\x1b[1;42m";
const COL_YELLOW: &str = "\x1b[1;43m";
const COL_BLUE: &str = "\x1b[1;44m";
const COL_BLACK: &str = "\x1b[1;40m";

#[derive(Debug, Clone, PartialEq, Eq)]
enum DominoColor {
    Unused,
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
    highlight: Vec<u64>,
}

impl DominoArea {
    fn create_empty(rows: u64, cols: u64) -> DominoArea {
        DominoArea {
            rows,
            cols,
            cells: (0..rows * cols).map(|_| DominoColor::Empty).collect(),
            highlight: vec![],
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
    fn is_position_valid(&self, row: u64, col: u64) -> bool {
        row < self.rows && col < self.cols
    }
    fn get_near_cells(&self, row: u64, col: u64) -> Vec<u64> {
        vec![
            (row + 1, col),
            (row, col + 1),
            (row.saturating_sub(1), col),
            (row, col.saturating_sub(1)),
        ]
        .iter()
        .filter(|(r, c)| *r != row || *c != col)
        .filter(|(r, c)| self.is_position_valid(*r, *c))
        .map(|(r, c)| self.to_index(*r, *c))
        .collect()
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

    fn get_highlight_mut(&mut self) -> &mut Vec<u64> {
        &mut self.highlight
    }

    fn get_valid_colors(&self, indexes: &[u64]) -> Vec<DominoColor> {
        for index in indexes {
            let cell = self.get_cell_at_index(*index);
            let row = self.row_from_index(*index);
            let col = self.col_from_index(*index);
            let empty = &DominoColor::Empty;
            assert_eq!(cell, empty, "already colored cell ({row},{col})!");
        }
        let mut valid_colors = vec![
            DominoColor::Blue,
            DominoColor::Red,
            DominoColor::Green,
            DominoColor::Yellow,
        ];
        for index in indexes {
            let row = self.row_from_index(*index);
            let col = self.col_from_index(*index);
            for near_index in self.get_near_cells(row, col) {
                let near_row = self.row_from_index(near_index);
                let near_col = self.col_from_index(near_index);
                if !self.is_position_valid(near_row, near_col) {
                    continue;
                }
                let cell = self.get_cell(near_row, near_col);
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

impl Display for DominoArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut block = STR_BLOCK;
                if self.highlight.contains(&self.to_index(row, col)) {
                    block = STR_BLOCK_HIGHLIGHT;
                }
                match self.get_cell(row, col) {
                    DominoColor::Empty => write!(f, "{}{}{}", COL_WHITE, block, COL_CLEAN)?,
                    DominoColor::Red => write!(f, "{}{}{}", COL_RED, block, COL_CLEAN)?,
                    DominoColor::Green => write!(f, "{}{}{}", COL_GREEN, block, COL_CLEAN)?,
                    DominoColor::Yellow => write!(f, "{}{}{}", COL_YELLOW, block, COL_CLEAN)?,
                    DominoColor::Blue => write!(f, "{}{}{}", COL_BLUE, block, COL_CLEAN)?,
                    DominoColor::Unused => write!(f, "{}{}{}", COL_BLACK, block, COL_CLEAN)?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut domino_area = DominoArea::create_empty(3, 5);
    domino_area.cells[4] = DominoColor::Unused;
    domino_area.cells[9] = DominoColor::Unused;
    domino_area.cells[14] = DominoColor::Unused;
    println!("\n{domino_area}");

    let indexes = [[1, 2], [0, 5], [10, 11], [6, 7], [12, 13], [3, 8]];
    for index in indexes {
        let valid_colors = domino_area.get_valid_colors(&index);
        println!("{valid_colors:?}");
        *domino_area.get_highlight_mut() = index.to_vec();
        domino_area.set_valid_color(&index, valid_colors[0].clone());
        println!("{domino_area}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_draw_colors() {
        let mut domino_area = DominoArea::create_empty(3, 5);
        domino_area.cells[12] = DominoColor::Unused;
        domino_area.cells[13] = DominoColor::Unused;
        println!("\n{domino_area}");

        let indexes = [
            vec![1, 2, 3],
            vec![0, 5],
            vec![10, 11, 6],
            vec![4, 9, 14],
            vec![7, 8],
        ];
        for index in indexes {
            let valid_colors = domino_area.get_valid_colors(&index);
            println!("{valid_colors:?}");
            *domino_area.get_highlight_mut() = index.clone();
            domino_area.set_valid_color(&index, valid_colors[0].clone());
            println!("{domino_area}");
        }
    }

    #[test]
    pub fn test_utils() {
        let domino_area = DominoArea::create_empty(5, 4);

        assert!(domino_area.is_position_valid(4, 3));
        assert!(domino_area.is_position_valid(0, 0));
        assert!(!domino_area.is_position_valid(5, 3));
        assert!(!domino_area.is_position_valid(1, 4));

        let near_tests = [
            (vec![1, 4], (0, 0)),
            (vec![1, 4, 6, 9], (1, 1)),
            (vec![7, 10, 15], (2, 3)),
            (vec![13, 16, 18], (4, 1)),
            (vec![15, 18], (4, 3)),
        ];
        for (expected, cell) in near_tests {
            let mut actual = domino_area.get_near_cells(cell.0, cell.1);
            actual.sort();
            assert_eq!(expected, actual);
        }
    }
}
