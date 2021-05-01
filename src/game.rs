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
            println!("illegal move: {:?}", chess_move);
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
    use crate::Color::*;
    use crate::{prelude::*, Position};

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

    #[test]
    fn en_passant() {
        let mut game = setup_game_1();

        game.make_move(ChessMove::EnPassant {
            from: E5,
            to: D6,
            taken_index: D5,
        })
        .unwrap();
    }

    #[test]
    fn checked() {
        let mut game = setup_game_2();

        assert!(game.make_move(regular(F2, F3)).is_err()); // this move would normally be allowed, but whites king is in check
        game.make_move(regular(C2, C3)).unwrap(); // block the check
        game.make_move(regular(F7, F6)).unwrap();
        assert!(game.make_move(regular(C3, C4)).is_err()); // can't make this move because it would remove the block from earlier
    }

    /// Set up a game where en passant is possible
    fn setup_game_1() -> Game {
        let mut game = Game::new();
        game.make_move(regular(E2, E4)).unwrap();
        game.make_move(regular(H7, H6)).unwrap();
        game.make_move(regular(E4, E5)).unwrap();
        game.make_move(regular(D7, D5)).unwrap();
        game
    }

    /// Set up a game where the white king is in check
    fn setup_game_2() -> Game {
        let mut game = Game::new();
        game.make_move(regular(E2, E4)).unwrap();
        game.make_move(regular(E7, E5)).unwrap();
        game.make_move(regular(D2, D4)).unwrap();
        game.make_move(regular(F8, B4)).unwrap();
        game
    }

    fn regular(from: Position, to: Position) -> ChessMove {
        ChessMove::Regular { from, to }
    }
}
