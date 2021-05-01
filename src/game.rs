use crate::{Board, Color};

pub struct Game {
    current_player: Color,
    board: Board,
}

impl Game {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Color {
        self.current_player
    }
}
