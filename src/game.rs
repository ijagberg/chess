use crate::{
    chess_move::{ChessMove, MoveManager},
    Board, Color,
};

pub struct Game {
    current_player: Color,
    move_manager: MoveManager,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::default();
        let current_player = Color::White;
        Self {
            current_player,
            move_manager: MoveManager::new(&board, current_player),
            board,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Color {
        self.current_player
    }

    pub fn get_moves(&self) -> &Vec<ChessMove> {
        self.move_manager.get_legal_moves()
    }

    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), ()> {
        if !self.move_manager.is_legal(chess_move) {
            Err(())
        } else {
            self.move_manager.make_move(&mut self.board, chess_move);
            Ok(())
        }
    }
}
