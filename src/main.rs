use std::fmt::Display;

#[derive(Debug, Default)]
enum DominoCell {
    #[default]
    Empty,
}

#[derive(Debug)]
struct DominoArea {
    cells: Vec<DominoCell>,
}

impl DominoArea {
    fn create_empty(rows: u64, cols: u64) -> DominoArea {
        DominoArea {
            cells: (0..rows * cols).map(|_| DominoCell::Empty).collect(),
        }
    }
}

impl Display for DominoCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for DominoArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

fn main() {
    println!("{}", DominoArea::create_empty(4, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_draw() {
        let actual = DominoArea::create_empty(3, 5).to_string();
        let expected = "";
        assert_eq!(actual, expected);
    }
}
