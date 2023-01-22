use bitboard::Position;

use crate::{
    chess_board::ChessBoard,
    chess_move::{ChessMove, MoveManager},
    Color,
};

pub struct Game {
    current_player: Color,
    move_manager: MoveManager,
    board: ChessBoard,
    bitboard: ChessBoard,
}

impl Game {
    pub fn new() -> Self {
        let board = ChessBoard::new();
        let current_player = Color::White;
        let mut move_manager = MoveManager::new(&board);
        move_manager.evaluate_legal_moves(&board, current_player);
        Self {
            current_player,
            move_manager,
            board,
            bitboard: ChessBoard::new(),
        }
    }

    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    pub fn current_player(&self) -> Color {
        self.current_player
    }

    pub fn get_moves(&self) -> &Vec<ChessMove> {
        self.move_manager.get_legal_moves()
    }

    pub fn get_moves_from(&self, from: Position) -> Vec<ChessMove> {
        let moves = self.move_manager.get_legal_moves();
        let mut moves_from = Vec::new();
        for &chess_move in moves {
            if chess_move.from() == from {
                moves_from.push(chess_move);
            }
        }

        moves_from
    }

    pub fn is_over(&self) -> bool {
        self.move_manager.get_legal_moves().is_empty()
    }

    pub fn winner(&self) -> Option<Color> {
        if self.is_over() {
            Some(self.current_player.opponent())
        } else {
            None
        }
    }

    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        if self.is_over() {
            Err("game is over")
        } else if !self.move_manager.is_legal(chess_move) {
            Err("illegal move")
        } else {
            self.move_manager
                .make_move(&mut self.board, self.current_player, chess_move);
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
    use crate::{chess_move::PromotionPiece, Color::*};
    use bitboard::*;
    use std::collections::HashSet;

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
            taken_original_index: D7,
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

    #[test]
    fn promotion() {
        let mut game = setup_promotion_game();

        game.make_move(ChessMove::Promotion {
            from: A7,
            to: B8,
            piece: PromotionPiece::Queen,
        })
        .unwrap();
    }

    #[test]
    fn castle() {
        let mut game = setup_castle_game();

        game.make_move(ChessMove::Castle {
            rook_from: H1,
            rook_to: F1,
            king_from: E1,
            king_to: G1,
        })
        .unwrap();

        let mut game = setup_castle_game();
        game.make_move(regular(D2, D3)).unwrap();
        game.make_move(regular(G5, G2)).unwrap();
        // queen is checking F1 and G1, preventing castling
        assert!(game
            .make_move(ChessMove::Castle {
                rook_from: H1,
                rook_to: F1,
                king_from: E1,
                king_to: G1,
            })
            .is_err());
    }

    #[test]
    fn immortal_game() {
        let mut game = Game::new();
        game.make_move(regular(E2, E4)).unwrap();
        game.make_move(regular(E7, E5)).unwrap();
        game.make_move(regular(F2, F4)).unwrap();
        game.make_move(regular(E5, F4)).unwrap();
        game.make_move(regular(F1, C4)).unwrap();
        game.make_move(regular(D8, H4)).unwrap();
        game.make_move(regular(E1, F1)).unwrap();
        game.make_move(regular(B7, B5)).unwrap();
        game.make_move(regular(C4, B5)).unwrap();
        game.make_move(regular(G8, F6)).unwrap();
        game.make_move(regular(G1, F3)).unwrap();
        game.make_move(regular(H4, H6)).unwrap();
        game.make_move(regular(D2, D3)).unwrap();
        game.make_move(regular(F6, H5)).unwrap();
        game.make_move(regular(F3, H4)).unwrap();
        game.make_move(regular(H6, G5)).unwrap();
        game.make_move(regular(H4, F5)).unwrap();
        game.make_move(regular(C7, C6)).unwrap();
        game.make_move(regular(G2, G4)).unwrap();
        game.make_move(regular(H5, F6)).unwrap();
        game.make_move(regular(H1, G1)).unwrap();
        game.make_move(regular(C6, B5)).unwrap();
        game.make_move(regular(H2, H4)).unwrap();
        game.make_move(regular(G5, G6)).unwrap();
        game.make_move(regular(H4, H5)).unwrap();
        game.make_move(regular(G6, G5)).unwrap();
        game.make_move(regular(D1, F3)).unwrap();
        game.make_move(regular(F6, G8)).unwrap();
        game.make_move(regular(C1, F4)).unwrap();
        game.make_move(regular(G5, F6)).unwrap();
        game.make_move(regular(B1, C3)).unwrap();
        game.make_move(regular(F8, C5)).unwrap();
        game.make_move(regular(C3, D5)).unwrap();
        game.make_move(regular(F6, B2)).unwrap();
        game.make_move(regular(F4, D6)).unwrap();
        game.make_move(regular(C5, G1)).unwrap();
        game.make_move(regular(E4, E5)).unwrap();
        game.make_move(regular(B2, A1)).unwrap();
        game.make_move(regular(F1, E2)).unwrap();
        game.make_move(regular(B8, A6)).unwrap();
        game.make_move(regular(F5, G7)).unwrap();
        game.make_move(regular(E8, D8)).unwrap();
        game.make_move(regular(F3, F6)).unwrap();
        game.make_move(regular(G8, F6)).unwrap();
        game.make_move(regular(D6, E7)).unwrap();
        assert_eq!(game.winner(), Some(Color::White));
    }

    #[test]
    fn wiede_vs_alphonse_goetz() {
        // https://www.chessgames.com/perl/chessgame?gid=1075778
        let mut game = Game::new();
        game.make_move(regular(E2, E4)).unwrap();
        game.make_move(regular(E7, E5)).unwrap();
        game.make_move(regular(F2, F4)).unwrap();
        game.make_move(regular(E5, F4)).unwrap();
        game.make_move(regular(B2, B3)).unwrap();
        game.make_move(regular(D8, H4)).unwrap();
        game.make_move(regular(G2, G3)).unwrap();
        game.make_move(regular(F4, G3)).unwrap();
        game.make_move(regular(H2, H3)).unwrap();
        game.make_move(regular(G3, G2)).unwrap();
        game.make_move(regular(E1, E2)).unwrap();
        game.make_move(regular(H4, E4)).unwrap();
        game.make_move(regular(E2, F2)).unwrap();
        game.make_move(ChessMove::Promotion {
            from: G2,
            to: H1,
            piece: PromotionPiece::Knight,
        })
        .unwrap();
        assert_eq!(game.winner(), Some(Color::Black));
    }

    #[test]
    fn heinrich_lohmann_vs_rudolf_teschner() {
        // https://www.chessgames.com/perl/chessgame?gid=1250788
        let mut game = Game::new();
        game.make_move(regular(E2, E4)).unwrap();
        game.make_move(regular(E7, E6)).unwrap();
        game.make_move(regular(D2, D4)).unwrap();
        game.make_move(regular(D7, D5)).unwrap();
        game.make_move(regular(B1, C3)).unwrap();
        game.make_move(regular(D5, E4)).unwrap();
        game.make_move(regular(C3, E4)).unwrap();
        game.make_move(regular(B8, D7)).unwrap();
        game.make_move(regular(G1, F3)).unwrap();
        game.make_move(regular(G8, F6)).unwrap();
        game.make_move(regular(F3, G5)).unwrap();
        game.make_move(regular(F8, E7)).unwrap();
        game.make_move(regular(G5, F7)).unwrap();
        game.make_move(regular(E8, F7)).unwrap();
        game.make_move(regular(E4, G5)).unwrap();
        game.make_move(regular(F7, G8)).unwrap();
        game.make_move(regular(G5, E6)).unwrap();
        game.make_move(regular(D8, E8)).unwrap();
        game.make_move(regular(E6, C7)).unwrap();
        game.make_move(regular(E7, B4)).unwrap();
        assert_eq!(game.winner(), Some(Color::Black));
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

    /// Set up a game where white can promote a pawn
    fn setup_promotion_game() -> Game {
        let mut game = Game::new();
        game.make_move(regular(B2, B4)).unwrap();
        game.make_move(regular(A7, A5)).unwrap();
        game.make_move(regular(B4, A5)).unwrap();
        game.make_move(regular(B7, B6)).unwrap();
        game.make_move(regular(A5, A6)).unwrap();
        game.make_move(regular(B6, B5)).unwrap();
        game.make_move(regular(A6, A7)).unwrap();
        game.make_move(regular(B5, B4)).unwrap();
        game
    }

    /// Set up a game where white can castle
    fn setup_castle_game() -> Game {
        let mut game = Game::new();
        game.make_move(regular(E2, E3)).unwrap();
        game.make_move(regular(E7, E5)).unwrap();
        game.make_move(regular(F1, E2)).unwrap();
        game.make_move(regular(D8, G5)).unwrap();
        game.make_move(regular(G1, F3)).unwrap();
        game.make_move(regular(A7, A6)).unwrap();

        game
    }

    fn regular(from: Position, to: Position) -> ChessMove {
        ChessMove::Regular { from, to }
    }
}
