use crate::{ChessBoard, ChessIndex, File, FileIter, Rank, RankIter};

impl ChessBoard {
    #[must_use]
    pub fn whites_perspective(&self) -> String {
        let mut lines = Vec::new();

        for rank in RankIter::start_at(Rank::Eighth).rev() {
            let mut pieces = Vec::new();
            for file in FileIter::start_at(File::A) {
                let chess_index = ChessIndex::from((file, rank));
                let output = match self[chess_index].piece() {
                    Some(p) => format!("{}", p),
                    None => " ".to_string(),
                };

                pieces.push(output);
            }

            let mut line = format!("{}│ ", rank);
            line.push_str(&pieces.join(" │ "));
            line.push_str(" │\n");

            lines.push(line);
        }

        let mut output = String::from("   a   b   c   d   e   f   g   h  \n");
        output.push_str(" ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
        output.push_str(&lines.join(" ├───┼───┼───┼───┼───┼───┼───┼───┤\n"));
        output.push_str(" └───┴───┴───┴───┴───┴───┴───┴───┘");
        output
    }

    #[must_use]
    pub fn blacks_perspective(&self) -> String {
        let mut lines = Vec::new();

        for rank in RankIter::start_at(Rank::First) {
            let mut pieces = Vec::new();
            for file in FileIter::start_at(File::H).rev() {
                let chess_index = ChessIndex::from((file, rank));
                let output = match self[chess_index].piece() {
                    Some(p) => format!("{}", p),
                    None => " ".to_string(),
                };

                pieces.push(output);
            }

            let mut line = format!("{}│ ", rank);
            line.push_str(&pieces.join(" │ "));
            line.push_str(" │\n");

            lines.push(line);
        }

        let mut output = String::from("   h   g   f   e   d   c   b   a  \n");
        output.push_str(" ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
        output.push_str(&lines.join(" ├───┼───┼───┼───┼───┼───┼───┼───┤\n"));
        output.push_str(" └───┴───┴───┴───┴───┴───┴───┴───┘");
        output
    }
}
