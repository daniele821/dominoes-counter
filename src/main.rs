#![allow(unused)]

use std::fmt::Display;

const STR_BLOCK: &str = "▀ ";
const COL_CLEAN: &str = "\x1b[0m";
const COL_RED: &str = "\x1b[1;31m";
const COL_GREEN: &str = "\x1b[1;32m";
const COL_YELLOW: &str = "\x1b[1;33m";
const COL_BLUE: &str = "\x1b[1;34m";

#[derive(Debug, Default)]
enum DominoColor {
    #[default]
    Empty,
    Color1,
    Color2,
    Color3,
    Color4,
}

#[derive(Debug)]
struct DominoArea {
    rows: u64,
    cols: u64,
    cells: Vec<DominoColor>,
}

#[derive(Debug)]
struct CellPos {
    row: u64,
    col: u64,
}

impl DominoArea {
    fn create_empty(rows: u64, cols: u64) -> DominoArea {
        DominoArea {
            rows,
            cols,
            cells: (0..rows * cols).map(|_| DominoColor::Empty).collect(),
        }
    }

    fn find_valid_color(&self, cell_pos: Vec<CellPos>) -> DominoColor {
        todo!("implement algorithm to find valid color!");
    }
}

impl Display for DominoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DominoColor::Empty => write!(f, "{}", STR_BLOCK),
            DominoColor::Color1 => write!(f, "{}{}{}", COL_RED, STR_BLOCK, COL_CLEAN),
            DominoColor::Color2 => write!(f, "{}{}{}", COL_GREEN, STR_BLOCK, COL_CLEAN),
            DominoColor::Color3 => write!(f, "{}{}{}", COL_YELLOW, STR_BLOCK, COL_CLEAN),
            DominoColor::Color4 => write!(f, "{}{}{}", COL_BLUE, STR_BLOCK, COL_CLEAN),
        }
    }
}

impl Display for DominoArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index: usize = (row * self.cols + col).try_into().unwrap();
                write!(f, "{}", self.cells[index].to_string())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut i = DominoArea::create_empty(3, 5);
    i.cells[3] = DominoColor::Color4;
    i.cells[4] = DominoColor::Color3;
    i.cells[7] = DominoColor::Color2;
    i.cells[8] = DominoColor::Color1;
    println!("{i}");
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
