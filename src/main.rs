use std::fmt::Display;

const STR_HALFBLOCK: &str = "▀";
const COL_CLEAN: &str = "\x1b[0m";
const COL_BOLD: &str = "\x1b[1m";
const COL_RED: &str = "\x1b[31m";
const COL_LGREEN: &str = "\x1b[32m";
const COL_YELLOW: &str = "\x1b[33m";
const COL_BLUE: &str = "\x1b[34m";
const COL_PURPLE: &str = "\x1b[35m";
const COL_GREEN: &str = "\x1b[36m";
const COL_WHITE: &str = "\x1b[37m";

#[derive(Debug, Default)]
enum DominoCell {
    #[default]
    Empty,
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
            DominoCell::Empty => write!(f, "{} ", STR_HALFBLOCK),
        }
    }
}

impl Display for DominoArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.cells[(row * col + col) as usize].to_string())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let i = DominoArea::create_empty(3, 5);
    println!("{}{}{}{}", COL_RED, COL_BOLD, i.to_string(), COL_CLEAN);
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
