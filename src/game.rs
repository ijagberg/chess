use crate::{
    chess_board::ChessBoard,
    chess_move::{ChessMove, MoveManager},
    Color,
};
use bitboard::{File, Position, Rank};
use std::collections::HashSet;

/// A game of chess.
#[derive(Debug)]
pub struct Game {
    current_player: Color,
    move_manager: MoveManager,
    board: ChessBoard,
}

impl Game {
    /// Create a new chess game, with pieces on their standard starting positions.
    pub fn new() -> Self {
        let board = ChessBoard::new();
        let current_player = Color::White;
        let mut move_manager = MoveManager::default();
        move_manager.evaluate_legal_moves(&board, current_player);
        Self::construct(current_player, move_manager, board)
    }

    fn construct(current_player: Color, move_manager: MoveManager, board: ChessBoard) -> Self {
        Self {
            current_player,
            move_manager,
            board,
        }
    }

    /// Get a reference to the `ChessBoard` of the game.
    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    /// Get the player whose turn it is.
    pub fn current_player(&self) -> Color {
        self.current_player
    }

    /// Get a list of all possible moves for the current player.
    pub fn get_moves(&self) -> &Vec<ChessMove> {
        self.move_manager.get_legal_moves()
    }

    /// Get a list of all possible moves for the current player from `from`.
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

    /// Returns `true` if the game is over (if a checkmate or stalemate has been reached).
    pub fn is_over(&self) -> bool {
        self.move_manager.get_legal_moves().is_empty()
    }

    /// Returns the result of the game, or `None` if the game is not over.
    pub fn game_result(&self) -> Option<GameOver> {
        if self.is_over() {
            if self
                .move_manager
                .is_in_check(&self.board, self.current_player())
            {
                Some(GameOver::Winner(self.current_player().opponent()))
            } else {
                Some(GameOver::Draw)
            }
        } else {
            None
        }
    }

    /// Make a move.
    ///
    /// # Returns
    /// * `Ok` if the move was successful.
    /// * `Err` if the game is over.
    /// * `Err` if the move is not legal.
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        if self.is_over() {
            Err("game is over")
        } else if !self.move_manager.is_legal(chess_move) {
            Err("illegal move")
        } else {
            self.move_manager
                .make_move(&mut self.board, self.current_player, chess_move);
            dbg!("move was made", chess_move);
            self.current_player = self.current_player.opponent();
            self.move_manager
                .evaluate_legal_moves(&self.board, self.current_player);

            Ok(())
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, ()> {
        // FEN: 8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50
        let parts: Vec<_> = fen.split(' ').collect();

        if parts.len() != 6 {
            return Err(());
        }

        // first part is board setup
        let board = ChessBoard::from_fen(parts[0])?;

        // second part is current player
        let current_player = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(()),
        };

        // the third part contains castling rights
        let castling: HashSet<char> = parts[2].chars().collect();
        let white_kingside_castle = castling.contains(&'K');
        let white_queenside_castle = castling.contains(&'Q');
        let black_kingside_castle = castling.contains(&'k');
        let black_queenside_castle = castling.contains(&'q');

        // the fourth part is en passant target square
        let (en_passant_possible_for_black, en_passant_possible_for_white) = if parts[3] != "-" {
            let position: Position = parts[3].parse().map_err(|_| ())?;
            match position.rank() {
                Rank::Three => ((Some(position), None)),
                Rank::Six => (None, Some(position)),
                _ => (None, None),
            }
        } else {
            (None, None)
        };

        // the fifth part contains the number of halfmoves since the last capture or pawn advancement (for 50-move rule)
        let half_moves = parts[4].parse().map_err(|_| ())?;

        // the sixth part contains the number of fullmoves, starting at 1
        let full_moves = parts[5].parse().map_err(|_| ())?;

        let mut move_manager = MoveManager::new(
            vec![],
            vec![],
            en_passant_possible_for_white,
            en_passant_possible_for_black,
            white_kingside_castle,
            white_queenside_castle,
            black_kingside_castle,
            black_queenside_castle,
            half_moves,
            full_moves,
        );

        move_manager.evaluate_legal_moves(&board, current_player);

        Ok(Self::construct(current_player, move_manager, board))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOver {
    Winner(Color),
    Draw,
}

impl GameOver {
    pub fn unwrap_winner(self) -> Color {
        match self {
            GameOver::Winner(w) => w,
            GameOver::Draw => panic!("called `GameOver::unwrap_winner()` on a `Draw` value"),
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

        dbg!(&game);
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

        dbg!(&game.get_moves());

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
        assert_eq!(game.game_result().unwrap().unwrap_winner(), Color::White);
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
        assert_eq!(game.game_result().unwrap().unwrap_winner(), Color::Black);
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
        dbg!(&game.get_moves());
        game.make_move(regular(F7, G8)).unwrap();
        game.make_move(regular(G5, E6)).unwrap();
        game.make_move(regular(D8, E8)).unwrap();
        game.make_move(regular(E6, C7)).unwrap();
        game.make_move(regular(E7, B4)).unwrap();
        assert_eq!(game.game_result().unwrap().unwrap_winner(), Color::Black);
    }

    #[test]
    fn stalemate_test() {
        let mut game = Game::new();
        // this is the fastest known stalemate
        // https://www.chess.com/forum/view/game-showcase/fastest-stalemate-known-in-chess
        for (from, to) in [
            (E2, E3),
            (A7, A5),
            (D1, H5),
            (A8, A6),
            (H5, A5),
            (H7, H5),
            (H2, H4),
            (A6, H6),
            (A5, C7),
            (F7, F6),
            (C7, D7),
            (E8, F7),
            (D7, B7),
            (D8, D3),
            (B7, B8),
            (D3, H7),
            (B8, C8),
            (F7, G6),
            (C8, E6),
        ] {
            game.make_move(regular(from, to)).unwrap();
        }
        assert!(game.is_over());
        assert_eq!(game.game_result().unwrap(), GameOver::Draw);
    }

    #[test]
    fn from_fen_test() {
        use ChessMove::*;
        let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
        let game = Game::from_fen(fen).unwrap();
        assert!(game.current_player().is_black());
        dbg!(game.board());
        assert_eq!(
            game.move_manager,
            MoveManager::new(
                vec![],
                vec![
                    Regular { from: F7, to: F6 },
                    Regular { from: F7, to: G6 },
                    Regular { from: F7, to: E7 },
                    Regular { from: F7, to: G7 },
                    Regular { from: F7, to: E8 },
                    Regular { from: F7, to: F8 },
                    Regular { from: F7, to: G8 },
                ],
                None,
                None,
                false,
                false,
                false,
                false,
                99,
                50
            )
        );
    }

    /// Set up a game where en passant is possible
    fn setup_game_1() -> Game {
        let mut game = Game::new();
        dbg!(&game);
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
