use crate::{chess_board::ChessBoard, game::Game, piece::PieceType, Color, Piece};
use bitboard::*;
use std::option::Option;

pub const KNIGHT_OFFSETS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (1, -2),
    (-1, -2),
];

pub const KING_OFFSETS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ChessMove {
    Regular {
        from: Position,
        to: Position,
    },
    EnPassant {
        from: Position,
        to: Position,
        taken_original_index: Position,
        taken_index: Position,
    },
    Promotion {
        from: Position,
        to: Position,
        piece: PromotionPiece,
    },
    Castle {
        rook_from: Position,
        rook_to: Position,
        king_from: Position,
        king_to: Position,
    },
}

impl ChessMove {
    pub(crate) fn from(&self) -> Position {
        *match self {
            ChessMove::Regular { from, to } => from,
            ChessMove::EnPassant {
                from,
                to,
                taken_index,
                taken_original_index,
            } => from,
            ChessMove::Promotion { from, to, piece } => from,
            ChessMove::Castle {
                rook_from,
                rook_to,
                king_from,
                king_to,
            } => king_from,
        }
    }

    pub(crate) fn to(&self) -> Position {
        *match self {
            ChessMove::Regular { from: _, to } => to,
            ChessMove::EnPassant {
                from: _,
                to,
                taken_index,
                taken_original_index,
            } => to,
            ChessMove::Promotion {
                from: _,
                to,
                piece: _,
            } => to,
            ChessMove::Castle {
                rook_from: _,
                rook_to: _,
                king_from: _,
                king_to,
            } => king_to,
        }
    }

    pub(crate) fn promotion_moves(from: Position, to: Position) -> Vec<ChessMove> {
        use PromotionPiece as PP;
        [PP::Bishop, PP::Knight, PP::Rook, PP::Queen]
            .iter()
            .map(|&piece| ChessMove::Promotion { from, to, piece })
            .collect()
    }

    /// Returns `true` if the chess move is [`Regular`].
    ///
    /// [`Regular`]: ChessMove::Regular
    #[must_use]
    pub fn is_regular(&self) -> bool {
        matches!(self, Self::Regular { .. })
    }

    /// Returns `true` if the chess move is [`EnPassant`].
    ///
    /// [`EnPassant`]: ChessMove::EnPassant
    #[must_use]
    pub fn is_en_passant(&self) -> bool {
        matches!(self, Self::EnPassant { .. })
    }

    /// Returns `true` if the chess move is [`Promotion`].
    ///
    /// [`Promotion`]: ChessMove::Promotion
    #[must_use]
    pub fn is_promotion(&self) -> bool {
        matches!(self, Self::Promotion { .. })
    }

    /// Returns `true` if the chess move is [`Castle`].
    ///
    /// [`Castle`]: ChessMove::Castle
    #[must_use]
    pub fn is_castle(&self) -> bool {
        matches!(self, Self::Castle { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionPiece {
    pub(crate) fn create_piece(&self, color: Color) -> Piece {
        match self {
            PromotionPiece::Knight => Piece::knight(color),
            PromotionPiece::Bishop => Piece::bishop(color),
            PromotionPiece::Rook => Piece::rook(color),
            PromotionPiece::Queen => Piece::queen(color),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MoveManager {
    history: Vec<ChessBoard>,
    pub(crate) legal_moves: Vec<ChessMove>,
    en_passant_possible_for_white: Option<Position>,
    en_passant_possible_for_black: Option<Position>,
    white_kingside_castle: bool,
    white_queenside_castle: bool,
    black_kingside_castle: bool,
    black_queenside_castle: bool,
}

impl MoveManager {
    pub(crate) fn new() -> Self {
        let mut this = Self {
            history: vec![],
            legal_moves: vec![],
            en_passant_possible_for_white: None,
            en_passant_possible_for_black: None,
            white_kingside_castle: true,
            white_queenside_castle: true,
            black_kingside_castle: true,
            black_queenside_castle: true,
        };

        this
    }

    pub(crate) fn is_legal(&self, chess_move: ChessMove) -> bool {
        self.legal_moves.contains(&chess_move)
    }

    pub(crate) fn dry_run_move(
        &self,
        board: &mut ChessBoard,
        player: Color,
        chess_move: ChessMove,
    ) -> Option<Piece> {
        let taken_piece;
        match chess_move {
            ChessMove::Regular { from, to } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, piece);
                taken_piece = taken;
            }
            ChessMove::EnPassant {
                from,
                to,
                taken_original_index,
                taken_index,
            } => {
                let piece = board.take_piece(from).unwrap();
                board.set_piece(to, piece);
                let taken = board.take_piece(taken_index).unwrap();
                taken_piece = Some(taken);
            }
            ChessMove::Promotion {
                from,
                to,
                piece: promotion,
            } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, promotion.create_piece(player));
                taken_piece = taken;
            }
            ChessMove::Castle {
                rook_from,
                rook_to,
                king_from,
                king_to,
            } => {
                let rook = board.take_piece(rook_from).unwrap();
                board.set_piece(rook_to, rook);
                let king = board.take_piece(king_from).unwrap();
                board.set_piece(king_to, king);
                taken_piece = None;
            }
        }
        taken_piece
    }

    pub(crate) fn make_move(
        &mut self,
        board: &mut ChessBoard,
        player: Color,
        chess_move: ChessMove,
    ) -> Option<Piece> {
        if let ChessMove::Regular { from, to } = chess_move {
            if let Some(Piece {
                color,
                kind: PieceType::Pawn,
            }) = board.get_piece(from)
            {
                if from.file() == to.file() && from.manhattan_distance_to(to) == 2 {
                    match color {
                        Color::Black => {
                            self.en_passant_possible_for_white =
                                Some(Position::new(from.file(), from.rank().down().unwrap()));
                        }
                        Color::White => {
                            self.en_passant_possible_for_black =
                                Some(Position::new(from.file(), from.rank().up().unwrap()));
                        }
                    }
                }
            }
            if from == E1 {
                self.white_kingside_castle = false;
                self.white_queenside_castle = false;
            }
            if from == A1 {
                self.white_queenside_castle = false;
            }
            if from == A8 {
                self.white_kingside_castle = false;
            }
            if from == E8 {
                self.black_kingside_castle = false;
                self.black_queenside_castle = false;
            }
            if from == A8 {
                self.black_queenside_castle = false;
            }
            if from == H8 {
                self.black_kingside_castle = false;
            }
        }

        let taken_piece = self.dry_run_move(board, player, chess_move);

        self.history.push(*board);

        taken_piece
    }

    pub fn get_legal_moves(&self) -> &Vec<ChessMove> {
        &self.legal_moves
    }

    pub(crate) fn evaluate_legal_moves(&mut self, board: &ChessBoard, player: Color) {
        let mut legal_moves = Vec::with_capacity(60);
        for pos in board.get_occupancy_for_color(player).positions() {
            let mut legal_moves_from_pos = self.evaluate_legal_moves_from(board, pos, player);
            legal_moves.append(&mut legal_moves_from_pos);
        }

        let mut actual_legal_moves = Vec::with_capacity(60);
        for &legal_move in &legal_moves {
            let mut board_clone = board.clone();
            print!("testing move: {}, {:?}", player, legal_move);
            self.dry_run_move(&mut board_clone, player, legal_move);
            if !self.is_in_check(&board_clone, player) {
                println!(" LEGAL");
                actual_legal_moves.push(legal_move);
            } else {
                println!(" ILLEGAL");
            }
        }

        self.legal_moves = actual_legal_moves;
    }

    pub fn is_in_check(&self, board: &ChessBoard, player: Color) -> bool {
        match player {
            Color::Black => {
                let black_king = board
                    .get_bitboard(Color::Black, PieceType::King)
                    .first_position()
                    .unwrap();
                let attackers = self.get_attackers(board, black_king, Color::White);
                if attackers > 0 {
                    println!("black king is in check, attackers: {}", attackers);
                    return true;
                }
                false
            }
            Color::White => {
                dbg!(board.get_bitboard(Color::White, PieceType::King));
                let white_king = board
                    .get_bitboard(Color::White, PieceType::King)
                    .first_position()
                    .unwrap();
                dbg!(white_king);
                let attackers = self.get_attackers(board, white_king, Color::Black);
                if attackers > 0 {
                    println!("white king is in check, attackers: {}", attackers);
                    return true;
                }
                false
            }
        }
    }

    fn is_under_attack(&self, board: &ChessBoard, target: Position, attacker_color: Color) -> bool {
        use Color::*;
        use PieceType::*;

        self.get_attackers(board, target, attacker_color) > 0
    }

    fn get_attackers(
        &self,
        board: &ChessBoard,
        target: Position,
        attacker_color: Color,
    ) -> Bitboard {
        use Color::*;
        use PieceType::*;

        let mut attacker_bb = Bitboard::empty();
        for piece_type in PieceType::all_iter() {
            attacker_bb |= match (attacker_color, piece_type) {
                (Black, Pawn) => {
                    (Position::up_left(&target)
                        .map(|p| Bitboard::with_one(p))
                        .unwrap_or(Bitboard::empty())
                        | Position::up_right(&target)
                            .map(|p| Bitboard::with_one(p))
                            .unwrap_or(Bitboard::empty()))
                        & board.get_bitboard(Black, Pawn)
                }
                (Black, Knight) => {
                    Bitboard::knight_targets(target) & board.get_bitboard(Black, Knight)
                }
                (Black, Bishop) => {
                    Bitboard::bishop_targets(target, board.full_occupancy())
                        & board.get_bitboard(Black, Bishop)
                }
                (Black, Rook) => {
                    Bitboard::rook_targets(target, board.full_occupancy())
                        & board.get_bitboard(Black, Rook)
                }
                (Black, Queen) => {
                    Bitboard::queen_targets(target, board.full_occupancy())
                        & board.get_bitboard(Black, Queen)
                }
                (Black, King) => Bitboard::king_targets(target) & board.get_bitboard(Black, King),
                (White, Pawn) => {
                    (Position::down_left(&target)
                        .map(|p| Bitboard::with_one(p))
                        .unwrap_or(Bitboard::empty())
                        | Position::down_right(&target)
                            .map(|p| Bitboard::with_one(p))
                            .unwrap_or(Bitboard::empty()))
                        & board.get_bitboard(White, Pawn)
                }
                (White, Knight) => {
                    Bitboard::knight_targets(target) & board.get_bitboard(White, Knight)
                }
                (White, Bishop) => {
                    Bitboard::bishop_targets(target, board.full_occupancy())
                        & board.get_bitboard(White, Bishop)
                }
                (White, Rook) => {
                    Bitboard::rook_targets(target, board.full_occupancy())
                        & board.get_bitboard(White, Rook)
                }
                (White, Queen) => {
                    Bitboard::queen_targets(target, board.full_occupancy())
                        & board.get_bitboard(White, Queen)
                }
                (White, King) => Bitboard::king_targets(target) & board.get_bitboard(White, King),
            }
        }
        attacker_bb
    }

    fn evaluate_legal_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if let Some(piece) = board.get_piece(from) {
            if piece.color() == player {
                return match piece.kind() {
                    PieceType::Pawn => self.evaluate_legal_pawn_moves_from(board, from, player),
                    PieceType::Knight => self.evaluate_legal_knight_moves_from(board, from, player),
                    PieceType::Bishop => self.evaluate_legal_bishop_moves_from(board, from, player),
                    PieceType::Rook => self.evaluate_legal_rook_moves_from(board, from, player),
                    PieceType::Queen => self.evaluate_legal_queen_moves_from(board, from, player),
                    PieceType::King => self.evaluate_legal_king_moves_from(board, from, player),
                };
            }
        }

        println!("no piece on {from}");
        Vec::new()
    }

    fn evaluate_legal_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::One || from.rank() == Rank::Eight {
            // TODO: mark this as an error somehow?
            // There should never be a pawn on the first or eighth ranks.
            return Vec::new();
        }
        match player {
            Color::Black => self.evaluate_legal_black_pawn_moves_from(board, from),
            Color::White => self.evaluate_legal_white_pawn_moves_from(board, from),
        }
    }

    fn evaluate_legal_white_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> Vec<ChessMove> {
        let mut legal_moves = Vec::with_capacity(10);
        if from.rank() == Rank::Seven {
            // from here it's only possible to promote

            // check position in front
            let up = from.up().unwrap();
            if !board.has_piece_at(up) {
                legal_moves.append(&mut ChessMove::promotion_moves(from, up));
            }
            if let Some(up_left) = from.up_left() {
                if board.has_piece_of_color_at(Color::Black, up_left) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, up_left));
                }
            }
            if let Some(up_right) = from.up_right() {
                if board.has_piece_of_color_at(Color::Black, up_right) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, up_right));
                }
            }
        } else {
            let targets = Bitboard::white_pawn_targets(
                from,
                board.get_occupancy_for_color(Color::Black),
                board.get_occupancy_for_color(Color::White),
            ) & !board.get_occupancy_for_color(Color::White);
            for to in targets.positions() {
                legal_moves.push(ChessMove::Regular { from, to });
            }

            // en passant
            if let t @ Some(en_passant_target) = self.en_passant_possible_for_white {
                if from.rank() == Rank::Five && (from.up_left() == t || from.up_right() == t) {
                    legal_moves.push(ChessMove::EnPassant {
                        from,
                        to: en_passant_target,
                        taken_original_index: Position::new(en_passant_target.file(), Rank::Seven),
                        taken_index: Position::new(en_passant_target.file(), Rank::Five),
                    })
                }
            }
        }
        legal_moves
    }

    fn evaluate_legal_black_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> Vec<ChessMove> {
        let mut legal_moves = Vec::with_capacity(10);
        if from.rank() == Rank::Two {
            // from here it's only possible to promote

            // check position in front
            let down = from.down().unwrap();
            if !board.has_piece_at(down) {
                legal_moves.append(&mut ChessMove::promotion_moves(from, down));
            }
            if let Some(down_left) = from.down_left() {
                if board.has_piece_of_color_at(Color::White, down_left) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, down_left));
                }
            }
            if let Some(down_right) = from.down_right() {
                if board.has_piece_of_color_at(Color::White, down_right) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, down_right));
                }
            }
        } else {
            let targets = Bitboard::black_pawn_targets(
                from,
                board.get_occupancy_for_color(Color::White),
                board.get_occupancy_for_color(Color::Black),
            ) & !board.get_occupancy_for_color(Color::Black);
            for to in targets.positions() {
                legal_moves.push(ChessMove::Regular { from, to });
            }

            // en passant
            if let t @ Some(en_passant_target) = self.en_passant_possible_for_black {
                if from.rank() == Rank::Four && (from.down_left() == t || from.down_right() == t) {
                    legal_moves.push(ChessMove::EnPassant {
                        from,
                        to: en_passant_target,
                        taken_original_index: Position::new(en_passant_target.file(), Rank::Two),
                        taken_index: Position::new(en_passant_target.file(), Rank::Four),
                    })
                }
            }
        }
        legal_moves
    }

    fn evaluate_legal_king_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        use bitboard::{E1, E8};
        let targets = Bitboard::king_targets(from) & !board.get_occupancy_for_color(player);
        let mut legal_moves: Vec<ChessMove> = targets
            .positions()
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        match (player, from) {
            (Color::Black, E8) => {
                if let Some(mut castle_moves) = self.evaluate_black_castle(board) {
                    legal_moves.append(&mut castle_moves);
                }
            }
            (Color::White, E1) => {
                if let Some(mut castle_moves) = self.evaluate_white_castle(board) {
                    legal_moves.append(&mut castle_moves);
                }
            }
            _ => {}
        }
        legal_moves
    }

    fn evaluate_white_castle(&self, board: &ChessBoard) -> Option<Vec<ChessMove>> {
        use bitboard::*;
        dbg!("evaluating white castle");
        let mut moves = Vec::with_capacity(2);

        // check short castle
        if dbg!(self.white_kingside_castle)
            && !dbg!(board.has_piece_at(F1))
            && !dbg!(board.has_piece_at(G1))
            && !dbg!(self.is_under_attack(board, F1, Color::Black))
            && !dbg!(self.is_under_attack(board, G1, Color::Black))
        {
            moves.push(ChessMove::Castle {
                rook_from: H1,
                rook_to: F1,
                king_from: E1,
                king_to: G1,
            });
        }

        // check long castle
        if self.white_queenside_castle
            && !board.has_piece_at(D1)
            && !board.has_piece_at(C1)
            && !board.has_piece_at(B1)
            && !self.is_under_attack(board, D1, Color::Black)
            && !self.is_under_attack(board, C1, Color::Black)
            && !self.is_under_attack(board, B1, Color::Black)
        {
            moves.push(ChessMove::Castle {
                rook_from: A1,
                rook_to: D1,
                king_from: E1,
                king_to: C1,
            })
        }

        return Some(moves);
    }

    fn evaluate_black_castle(&self, board: &ChessBoard) -> Option<Vec<ChessMove>> {
        use bitboard::*;

        // has king moved?
        let mut moves = Vec::with_capacity(2);

        // check short castle
        if self.black_kingside_castle
            && !board.has_piece_at(F8)
            && !board.has_piece_at(G8)
            && !self.is_under_attack(board, F8, Color::White)
            && !self.is_under_attack(board, G8, Color::White)
        {
            moves.push(ChessMove::Castle {
                rook_from: H8,
                rook_to: F8,
                king_from: E8,
                king_to: G8,
            });
        }

        // check long castle
        if self.black_queenside_castle
            && !board.has_piece_at(D8)
            && !board.has_piece_at(C8)
            && !board.has_piece_at(B8)
            && !self.is_under_attack(board, D8, Color::White)
            && !self.is_under_attack(board, C8, Color::White)
            && !self.is_under_attack(board, B8, Color::White)
        {
            moves.push(ChessMove::Castle {
                rook_from: A8,
                rook_to: D8,
                king_from: E8,
                king_to: C8,
            })
        }

        return Some(moves);
    }

    fn evaluate_legal_knight_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::knight_targets(from) & !board.get_occupancy_for_color(player);

        let mut legal_moves = Vec::with_capacity(8);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to });
        }
        legal_moves
    }

    fn evaluate_legal_bishop_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::bishop_targets(from, board.full_occupancy())
            & !board.get_occupancy_for_color(player);
        let mut legal_moves = Vec::with_capacity(16);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to })
        }
        legal_moves
    }

    fn evaluate_legal_rook_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::rook_targets(from, board.full_occupancy())
            & !board.get_occupancy_for_color(player);

        let mut legal_moves = Vec::with_capacity(16);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to })
        }
        legal_moves
    }

    fn evaluate_legal_queen_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::queen_targets(from, board.full_occupancy())
            & !board.get_occupancy_for_color(player);
        let mut legal_moves = Vec::with_capacity(16);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to })
        }
        legal_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitboard::*;
    use Color::*;

    #[test]
    fn legal_moves() {
        let board = ChessBoard::new();
        let mut manager = MoveManager::new();
        manager.evaluate_legal_moves(&board, White);

        dbg!(&manager);

        let legal_moves: Vec<_> = manager
            .get_legal_moves()
            .iter()
            .map(|m| (m.from(), m.to()))
            .collect();

        dbg!(&legal_moves);

        assert_eq!(
            legal_moves,
            vec![
                (B1, A3),
                (B1, C3),
                (G1, F3),
                (G1, H3),
                (A2, A3),
                (A2, A4),
                (B2, B3),
                (B2, B4),
                (C2, C3),
                (C2, C4),
                (D2, D3),
                (D2, D4),
                (E2, E3),
                (E2, E4),
                (F2, F3),
                (F2, F4),
                (G2, G3),
                (G2, G4),
                (H2, H3),
                (H2, H4)
            ]
        );
    }

    #[test]
    fn bishop_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new();

        let bishop_moves_from_f4: Vec<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, F4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_f4, vec![E3, G3, E5, G5, D6, H6, C7]);

        let bishop_moves_from_c1: Vec<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, C1, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_c1, vec![]);
    }

    #[test]
    fn rook_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new();

        let rook_moves_from_c5: Vec<Position> = manager
            .evaluate_legal_rook_moves_from(&board, C5, Black)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            rook_moves_from_c5,
            vec![C2, C3, C4, A5, B5, D5, E5, F5, G5, H5, C6]
        );
    }

    #[test]
    fn knight_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new();

        let knight_moves_from_g4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, G4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_g4, vec![E3, E5, F6, H6]);

        let knight_moves_from_d4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, D4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_d4, vec![B3, F3, B5, F5, C6, E6]);
    }

    #[test]
    fn queen_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new();

        let queen_moves_from_a4: Vec<Position> = manager
            .evaluate_legal_queen_moves_from(&board, A4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            queen_moves_from_a4,
            vec![A3, B3, B4, C4, D4, E4, F4, G4, H4, A5, B5, A6, C6, A7, D7]
        );
    }

    #[test]
    /// Remove all pawns from a chessboard and check legal moves for white
    fn moves_test_1() {
        use ChessMove::*;
        let mut board = ChessBoard::new();
        for pos in [
            A2, B2, C2, D2, E2, F2, G2, H2, A7, B7, C7, D7, E7, F7, G7, H7,
        ] {
            board.take_piece(pos).unwrap();
        }

        dbg!(board);

        let mut move_manager = MoveManager::new();
        move_manager.evaluate_legal_moves(&board, White);
        let moves = move_manager.get_legal_moves();
        let expected = [
            // queenside rook can move up to and including A8 (which would take blacks queenside rook)
            Regular { from: A1, to: A2 },
            Regular { from: A1, to: A3 },
            Regular { from: A1, to: A4 },
            Regular { from: A1, to: A5 },
            Regular { from: A1, to: A6 },
            Regular { from: A1, to: A7 },
            Regular { from: A1, to: A8 },
            // queenside knight
            Regular { from: B1, to: D2 },
            Regular { from: B1, to: A3 },
            Regular { from: B1, to: C3 },
            // queenside (black square) bishop
            Regular { from: C1, to: B2 },
            Regular { from: C1, to: D2 },
            Regular { from: C1, to: A3 },
            Regular { from: C1, to: E3 },
            Regular { from: C1, to: F4 },
            Regular { from: C1, to: G5 },
            Regular { from: C1, to: H6 },
            // queen
            Regular { from: D1, to: C2 },
            Regular { from: D1, to: D2 },
            Regular { from: D1, to: E2 },
            Regular { from: D1, to: B3 },
            Regular { from: D1, to: D3 },
            Regular { from: D1, to: F3 },
            Regular { from: D1, to: A4 },
            Regular { from: D1, to: D4 },
            Regular { from: D1, to: G4 },
            Regular { from: D1, to: D5 },
            Regular { from: D1, to: H5 },
            Regular { from: D1, to: D6 },
            Regular { from: D1, to: D7 },
            Regular { from: D1, to: D8 },
            // king
            // Regular { from: E1, to: D2 }, // cant go to D2 because black queen is checking it
            Regular { from: E1, to: E2 },
            Regular { from: E1, to: F2 },
            // white square bishop
            Regular { from: F1, to: E2 },
            Regular { from: F1, to: G2 },
            Regular { from: F1, to: D3 },
            Regular { from: F1, to: H3 },
            Regular { from: F1, to: C4 },
            Regular { from: F1, to: B5 },
            Regular { from: F1, to: A6 },
            // kingside knight
            Regular { from: G1, to: E2 },
            Regular { from: G1, to: F3 },
            Regular { from: G1, to: H3 },
            // kingside rook
            Regular { from: H1, to: H2 },
            Regular { from: H1, to: H3 },
            Regular { from: H1, to: H4 },
            Regular { from: H1, to: H5 },
            Regular { from: H1, to: H6 },
            Regular { from: H1, to: H7 },
            Regular { from: H1, to: H8 },
        ];

        assert_eq!(moves.len(), expected.len());
        for (actual, expected) in moves.iter().zip(expected.iter()) {
            assert_eq!(actual, expected);
        }
    }
}
