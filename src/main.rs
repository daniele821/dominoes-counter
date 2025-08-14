use dominoes_counter::domino::{DominoArea, DominoColor};

fn main() {
    let mut domino_area = DominoArea::create_empty(3, 5);
    *domino_area.get_cell_at_index_mut(4) = DominoColor::Unused;
    *domino_area.get_cell_at_index_mut(9) = DominoColor::Unused;
    *domino_area.get_cell_at_index_mut(14) = DominoColor::Unused;
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
