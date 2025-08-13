use std::fmt::Display;

const STR_BLOCK: &str = "█";
const STR_HALFBLOCK: &str = "▀";

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
    println!("{}", i.to_string());
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
