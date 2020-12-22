use std::collections::HashSet;

use crate::{ChessBoard, ChessIndex, Color, File, FileIter, Rank, RankIter};

#[must_use]
pub fn whites_perspective(board: &ChessBoard, highlighted_squares: &HashSet<ChessIndex>) -> String {
    get_perspective(board, Color::White, highlighted_squares)
}

#[must_use]
pub fn blacks_perspective(board: &ChessBoard, highlighted_squares: &HashSet<ChessIndex>) -> String {
    get_perspective(board, Color::Black, highlighted_squares)
}

fn get_perspective(
    board: &ChessBoard,
    color: Color,
    highlighted_squares: &HashSet<ChessIndex>,
) -> String {
    let mut lines = Vec::new();

    for rank in color_rank(color) {
        let mut pieces = Vec::new();
        for file in color_file(color) {
            let index = ChessIndex::from((file, rank));
            let highlight = match highlighted_squares.contains(&index) {
                true => "X",
                false => " ",
            };
            let piece = match board[index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };
            let output = format!("{}{} ", highlight, piece);

            pieces.push(output);
        }

        let mut line = format!("{}│", rank);
        line.push_str(&pieces.join("│"));
        line.push_str("│\n");

        lines.push(line);
    }

    let mut output = match color {
        Color::Black => String::from("   h   g   f   e   d   c   b   a  \n"),
        Color::White => String::from("   a   b   c   d   e   f   g   h  \n"),
    };
    output.push_str(" ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
    output.push_str(&lines.join(" ├───┼───┼───┼───┼───┼───┼───┼───┤\n"));
    output.push_str(" └───┴───┴───┴───┴───┴───┴───┴───┘");
    output
}

fn color_rank(color: Color) -> Box<dyn Iterator<Item = Rank>> {
    match color {
        Color::Black => Box::new(RankIter::start_at(Rank::First)),
        Color::White => Box::new(RankIter::start_at(Rank::Eighth).rev()),
    }
}

fn color_file(color: Color) -> Box<dyn Iterator<Item = File>> {
    match color {
        Color::Black => Box::new(FileIter::start_at(File::H).rev()),
        Color::White => Box::new(FileIter::start_at(File::A)),
    }
}
