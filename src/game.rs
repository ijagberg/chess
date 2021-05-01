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
            dbg!("illegal move", chess_move);
            Err(())
        } else {
            self.move_manager.make_move(&mut self.board, chess_move);
            self.current_player = self.current_player.opponent();
            self.move_manager
                .evaluate_legal_moves(&self.board, self.current_player);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::Color::*;

    #[test]
    fn make_move() {
        let mut game = Game::new();
        let e2_e4 = ChessMove::Regular { from: E2, to: E4 };

        game.make_move(e2_e4).unwrap();

        assert_eq!(game.board.get_piece(E2), None);
        assert_eq!(game.board.get_piece(E4), Some(Piece::pawn(White)));

        let d7_d5 = ChessMove::Regular { from: D7, to: D5 };
        game.make_move(d7_d5).unwrap();

        assert_eq!(game.board.get_piece(D7), None);
        assert_eq!(game.board.get_piece(D5), Some(Piece::pawn(Black)));

        let illegal_move = ChessMove::Regular { from: H7, to: H6 };
        assert!(game.make_move(illegal_move).is_err());
    }
}
