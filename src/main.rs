use std::fmt::Display;

const STR_BLOCK: &str = "▀ ";
const COL_CLEAN: &str = "\x1b[0m";
const COL_RED: &str = "\x1b[1;31m";
const COL_GREEN: &str = "\x1b[1;32m";
const COL_YELLOW: &str = "\x1b[1;33m";
const COL_BLUE: &str = "\x1b[1;34m";

#[derive(Debug, Default)]
enum DominoCell {
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
    cells: Vec<DominoCell>,
}

impl DominoArea {
    fn create_empty(rows: u64, cols: u64) -> DominoArea {
        DominoArea {
            rows,
            cols,
            cells: (0..rows * cols).map(|_| DominoCell::Empty).collect(),
        }
    }
}

impl Display for DominoCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DominoCell::Empty => write!(f, "{}", STR_BLOCK),
            DominoCell::Color1 => write!(f, "{}{}{}", COL_RED, STR_BLOCK, COL_CLEAN),
            DominoCell::Color2 => write!(f, "{}{}{}", COL_GREEN, STR_BLOCK, COL_CLEAN),
            DominoCell::Color3 => write!(f, "{}{}{}", COL_YELLOW, STR_BLOCK, COL_CLEAN),
            DominoCell::Color4 => write!(f, "{}{}{}", COL_BLUE, STR_BLOCK, COL_CLEAN),
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
    i.cells[3] = DominoCell::Color4;
    i.cells[4] = DominoCell::Color3;
    i.cells[7] = DominoCell::Color2;
    i.cells[8] = DominoCell::Color1;
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
