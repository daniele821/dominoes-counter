const STR_BLOCK: &str = "  ";
const STR_BLOCK_HIGHLIGHT: &str = "✪ ";
const COL_TEXT: &str = "\x1b[30m";
const COL_CLEAN: &str = "\x1b[0m";
const COL_WHITE: &str = "\x1b[47m";
const COL_RED: &str = "\x1b[41m";
const COL_GREEN: &str = "\x1b[42m";
const COL_YELLOW: &str = "\x1b[43m";
const COL_BLUE: &str = "\x1b[44m";
const COL_BLACK: &str = "\x1b[40m";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DominoColor {
    Unused,
    Empty,
    Red,
    Green,
    Yellow,
    Blue,
}

#[derive(Debug)]
pub struct DominoArea {
    rows: u64,
    cols: u64,
    cells: Vec<DominoColor>,
}

impl DominoArea {
    pub fn create_empty(rows: u64, cols: u64) -> DominoArea {
        DominoArea {
            rows,
            cols,
            cells: (0..rows * cols).map(|_| DominoColor::Empty).collect(),
        }
    }

    pub fn row_from_index(&self, index: u64) -> u64 {
        index / self.cols
    }
    pub fn col_from_index(&self, index: u64) -> u64 {
        index % self.cols
    }
    pub fn to_index(&self, row: u64, col: u64) -> u64 {
        row * self.cols + col
    }

    pub fn is_position_valid(&self, row: u64, col: u64) -> bool {
        row < self.rows && col < self.cols
    }
    pub fn get_near_cells(&self, row: u64, col: u64) -> Vec<u64> {
        assert!(self.is_position_valid(row, col));
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

    pub fn get_cell_at_index(&self, index: u64) -> &DominoColor {
        self.cells.get(usize::try_from(index).unwrap()).unwrap()
    }
    pub fn get_cell(&self, row: u64, col: u64) -> &DominoColor {
        self.get_cell_at_index(self.to_index(row, col))
    }
    pub fn get_cell_at_index_mut(&mut self, index: u64) -> &mut DominoColor {
        self.cells.get_mut(usize::try_from(index).unwrap()).unwrap()
    }
    pub fn get_cell_mut(&mut self, row: u64, col: u64) -> &mut DominoColor {
        self.get_cell_at_index_mut(self.to_index(row, col))
    }

    pub fn get_valid_colors(&self, indexes: &[u64]) -> Vec<DominoColor> {
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
    pub fn set_valid_color(&mut self, indexes: &[u64], color: DominoColor) {
        for index in indexes {
            let cell = self.get_cell_at_index_mut(*index);
            *cell = color.clone();
        }
    }

    pub fn compute_empty_nears(&self) -> Vec<u64> {
        let mut result = Vec::<u64>::with_capacity(self.cells.len());
        for index in 0..self.cells.len() {
            let row = self.row_from_index(u64::try_from(index).unwrap());
            let col = self.col_from_index(u64::try_from(index).unwrap());
            let empty_nears = self
                .get_near_cells(row, col)
                .iter()
                .filter(|i| *self.get_cell_at_index(**i) == DominoColor::Empty)
                .count();
            result.push(u64::try_from(empty_nears).unwrap());
        }
        result
    }

    pub fn custom_fmt(&self, highlights: &[u64], custom: &[String]) -> String {
        let mut fmt = Vec::<String>::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut block = STR_BLOCK;
                if highlights.contains(&self.to_index(row, col)) {
                    block = STR_BLOCK_HIGHLIGHT;
                }
                if let Some(text) = custom.get(usize::try_from(self.to_index(row, col)).unwrap())
                    && !text.is_empty()
                {
                    block = text;
                }
                assert_eq!(2, block.chars().count());
                let shared_part = format!("{COL_TEXT}{block}{COL_CLEAN}");
                fmt.push(match self.get_cell(row, col) {
                    DominoColor::Empty => format!("{}{}", COL_WHITE, shared_part),
                    DominoColor::Red => format!("{}{}", COL_RED, shared_part),
                    DominoColor::Green => format!("{}{}", COL_GREEN, shared_part),
                    DominoColor::Yellow => format!("{}{}", COL_YELLOW, shared_part),
                    DominoColor::Blue => format!("{}{}", COL_BLUE, shared_part),
                    DominoColor::Unused => format!("{}{}", COL_BLACK, shared_part),
                })
            }
            fmt.push(format!("\n"));
        }
        fmt.join("")
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
        println!("\n{}", domino_area.custom_fmt(&[], &[]));

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
            domino_area.set_valid_color(&index, valid_colors[0].clone());
            println!("{}", domino_area.custom_fmt(&index, &[]));
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

    #[test]
    pub fn test_empty_nears() {
        let mut domino_area = DominoArea::create_empty(3, 4);
        *domino_area.get_cell_mut(0, 3) = DominoColor::Unused;
        *domino_area.get_cell_mut(1, 3) = DominoColor::Unused;
        println!("\n{}", domino_area.custom_fmt(&[], &[]));

        let tests = [
            (vec![1, 2, 1, 1, 2, 3, 3, 2, 2, 3, 3, 1], vec![0, 1]),
            (vec![1, 1, 0, 0, 2, 2, 2, 1, 2, 3, 2, 1], vec![2, 6]),
            (vec![0, 1, 0, 0, 1, 1, 2, 1, 1, 2, 2, 1], vec![4, 8]),
            (vec![0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0], vec![10, 11]),
            (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![5, 9]),
        ];

        for test in tests {
            let colors = domino_area.get_valid_colors(&test.1);
            domino_area.set_valid_color(&test.1, colors[0].clone());
            let expected = test.0;
            let actual = domino_area.compute_empty_nears();
            assert_eq!(expected, actual);
            println!("{actual:?}");
            let custom_text: Vec<_> = actual
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    if *domino_area.get_cell_at_index(u64::try_from(i).unwrap())
                        == DominoColor::Empty
                    {
                        format!("{n:<2}")
                    } else {
                        format!("")
                    }
                })
                .collect();
            println!("{}", domino_area.custom_fmt(&test.1, &custom_text));
        }
    }
}
