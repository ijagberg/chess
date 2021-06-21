use crate::{
    chess_move::{ChessMove, MoveManager},
    Board, Color, Position,
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
        let mut move_manager = MoveManager::new(&board);
        move_manager.evaluate_legal_moves(&board, current_player);
        Self {
            current_player,
            move_manager,
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

    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), String> {
        if self.is_over() {
            Err("game is over".to_string())
        } else if !self.move_manager.is_legal(chess_move) {
            Err(format!("{:?} is an illegal move", chess_move))
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

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{board, chess_move::PromotionPiece, Color::*};
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

    #[test]
    fn game_test() {
        let mut game = Game::new();

        let mut moves = Vec::new();
        moves.extend(
            [
                (G1, F3),
                (G8, F6),
                (D2, D4),
                (G7, G6),
                (E2, E3),
                (F8, G7),
                (C2, C4),
                (D7, D5),
                (B2, B3),
                (E7, E6),
                (C1, B2),
                (B8, C6),
                (F1, D3),
                (B7, B6),
            ]
            .iter()
            .map(|&(from, to)| regular(from, to)),
        );
        moves.push(ChessMove::Castle {
            rook_from: H1,
            rook_to: F1,
            king_from: E1,
            king_to: G1,
        });
        moves.extend(
            [(C8, B7), (B1, C3)]
                .iter()
                .map(|&(from, to)| regular(from, to)),
        );
        moves.push(ChessMove::Castle {
            rook_from: H8,
            rook_to: F8,
            king_from: E8,
            king_to: G8,
        });
        moves.extend(
            [
                (F1, E1),
                (F8, E8),
                (A1, C1),
                (E6, E5),
                (D4, E5),
                (C6, E5),
                (F3, E5),
                (E8, E5),
                (C3, D5),
                (E5, E8),
                (D5, F4),
                (D8, D7),
                (D1, D2),
                (F6, H5),
                (F4, H5),
                (G6, H5),
                (B2, G7),
                (D7, G4),
                (F2, F3),
                (G4, G7),
                (C4, C5),
                (B7, F3),
                (G1, H1),
                (F3, E4),
                (C5, B6),
                (A7, B6),
                (D3, E4),
                (E8, E4),
                (C1, C7),
                (G7, E5),
                (C7, C2),
                (E4, H4),
                (G2, G3),
                (E5, E4),
                (D2, G2),
                (E4, G2),
                (H1, G2),
                (H4, E4),
                (G2, F3),
                (A8, E8),
                (C2, C6),
                (E4, B4),
                (E1, C1),
                (E8, A8),
                (C6, C8),
                (A8, C8),
                (C1, C8),
                (G8, G7),
                (C8, C4),
                (B4, B5),
                (F3, F4),
                (G7, F6),
                (C4, C6),
                (F6, E7),
                (C6, C7),
                (E7, E6),
                (G3, G4),
                (B5, B4),
                (F4, G3),
                (B4, G4),
                (G3, F3),
                (G4, G1),
                (C7, C6),
                (E6, E5),
                (C6, B6),
                (G1, A1),
                (A2, A4),
                (A1, B1),
                (B6, B5),
                (E5, D6),
                (B5, H5),
                (B1, B3),
                (H5, H7),
                (D6, E6),
                (H7, H4),
                (F7, F5),
                (H4, F4),
                (E6, E5),
                (F3, G3),
                (B3, E3),
                (F4, F3),
                (E3, E4),
                (H2, H4),
                (E4, A4),
                (H4, H5),
                (A4, A6),
                (F3, F2),
                (A6, A3),
                (G3, H4),
                (A3, A6),
                (H4, G5),
                (A6, F6),
                (H5, H6),
                (F5, F4),
                (H6, H7),
                (F6, F8),
                (G5, G6),
                (E5, E4),
                (G6, G7),
                (F8, D8),
            ]
            .iter()
            .map(|&(from, to)| regular(from, to)),
        );
        moves.push(ChessMove::Promotion {
            from: H7,
            to: H8,
            piece: PromotionPiece::Queen,
        });
        moves.extend(
            [
                (D8, H8),
                (G7, H8),
                (E4, E3),
                (H8, G7),
                (E3, F2),
                (G7, F6),
                (F2, E3),
                (F6, F5),
                (F4, F3),
                (F5, G4),
                (F3, F2),
                (G4, G3),
            ]
            .iter()
            .map(|&(from, to)| regular(from, to)),
        );
        moves.push(ChessMove::Promotion {
            from: F2,
            to: F1,
            piece: PromotionPiece::Queen,
        });
        moves.extend(
            [
                (G3, G4),
                (F1, F2),
                (G4, G5),
                (F2, F3),
                (G5, G6),
                (F3, F4),
                (G6, G7),
                (F4, F5),
                (G7, G8),
                (F5, D7),
                (G8, F8),
                (D7, D6),
                (F8, G8),
                (D6, E7),
                (G8, H8),
                (E3, F4),
                (H8, G8),
                (F4, F5),
                (G8, H8),
                (F5, G6),
                (H8, G8),
                (E7, G7),
            ]
            .iter()
            .map(|&(from, to)| regular(from, to)),
        );

        for m in moves {
            game.make_move(m).unwrap();
        }

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
