use crate::{chess_board::ChessBoard, game::Game, piece::PieceType, Color, Piece};
use bitboard64::prelude::*;
use std::{collections::HashSet, option::Option, str::FromStr};

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

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum ChessMove {
    /// A regular chess move, moving a piece from one square to another.
    Regular { from: Position, to: Position },
    /// Holy hell.
    EnPassant {
        from: Position,
        to: Position,
        taken_original_index: Position,
        taken_index: Position,
    },
    /// Pawn promotion, including which piece was promoted to.
    Promotion {
        from: Position,
        to: Position,
        piece: PromotionPiece,
    },
    /// Castle
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

    /// Generate promotion moves.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Keeps track of legality of moves for a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MoveManager {
    board_history: Vec<ChessBoard>,
    move_history: Vec<ChessMove>,
    legal_moves: HashSet<ChessMove>,
    white_en_passant_target: Option<Position>,
    black_en_passant_target: Option<Position>,
    castling_rights: CastlingRights,
    half_moves: u32,
    full_moves: u32,
}

impl MoveManager {
    pub(crate) fn new(
        board_history: Vec<ChessBoard>,
        move_history: Vec<ChessMove>,
        legal_moves: HashSet<ChessMove>,
        white_en_passant_target: Option<Position>,
        black_en_passant_target: Option<Position>,
        castling_rights: CastlingRights,
        half_moves: u32,
        full_moves: u32,
    ) -> Self {
        Self {
            board_history,
            move_history,
            legal_moves,
            white_en_passant_target,
            black_en_passant_target,
            castling_rights,
            half_moves,
            full_moves,
        }
    }

    pub(crate) fn is_legal(&self, chess_move: ChessMove) -> bool {
        self.legal_moves.contains(&chess_move)
    }

    pub(crate) fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub(crate) fn white_en_passant_target(&self) -> Option<Position> {
        self.white_en_passant_target
    }

    pub(crate) fn black_en_passant_target(&self) -> Option<Position> {
        self.black_en_passant_target
    }

    pub(crate) fn half_moves(&self) -> u32 {
        self.half_moves
    }

    pub(crate) fn full_moves(&self) -> u32 {
        self.full_moves
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
        let mut moved_pawn = false;
        if let ChessMove::Regular { from, to } = chess_move {
            if let Some(Piece {
                color,
                kind: PieceType::Pawn,
            }) = board.get_piece(from)
            {
                // if we move a pawn, we reset 50-move counter later in the function
                moved_pawn = true;
                if from.file() == to.file() && from.manhattan_distance_to(to) == 2 {
                    match color {
                        Color::Black => {
                            self.white_en_passant_target =
                                Some(Position::new(from.file(), from.rank().down().unwrap()));
                        }
                        Color::White => {
                            self.black_en_passant_target =
                                Some(Position::new(from.file(), from.rank().up().unwrap()));
                        }
                    }
                }
            }
            self.update_castling_rights(from);
        }

        let taken_piece = self.dry_run_move(board, player, chess_move);

        if moved_pawn
            || chess_move.is_en_passant()
            || chess_move.is_promotion()
            || taken_piece.is_some()
        {
            // en passant and promotion moves a pawn, which resets 50 move rule
            // and so does taking a piece
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        match player {
            Color::Black => {
                self.full_moves += 1;
                self.black_en_passant_target = None;
            }
            Color::White => {
                self.white_en_passant_target = None;
            }
        }

        self.board_history.push(*board);

        taken_piece
    }

    pub fn get_legal_moves(&self) -> &HashSet<ChessMove> {
        &self.legal_moves
    }

    fn update_castling_rights(&mut self, from: Position) {
        if from == E1 {
            *self.castling_rights.white_kingside_mut() = false;
            *self.castling_rights.white_queenside_mut() = false;
        }
        if from == A1 {
            *self.castling_rights.white_queenside_mut() = false;
        }
        if from == A8 {
            *self.castling_rights.white_kingside_mut() = false;
        }
        if from == E8 {
            *self.castling_rights.black_kingside_mut() = false;
            *self.castling_rights.black_queenside_mut() = false;
        }
        if from == A8 {
            *self.castling_rights.black_queenside_mut() = false;
        }
        if from == H8 {
            *self.castling_rights.black_kingside_mut() = false;
        }
    }

    pub(crate) fn evaluate_legal_moves(&mut self, board: &ChessBoard, player: Color) {
        let mut legal_moves = Vec::with_capacity(60);
        for pos in board.get_occupancy_for_color(player).positions() {
            let legal_moves_from_pos = self.evaluate_legal_moves_from(board, pos, player);
            legal_moves.extend(legal_moves_from_pos);
        }

        let mut actual_legal_moves = HashSet::with_capacity(60);
        for &legal_move in &legal_moves {
            let mut board_clone = board.clone();
            self.dry_run_move(&mut board_clone, player, legal_move);
            if !self.is_in_check(&board_clone, player) {
                actual_legal_moves.insert(legal_move);
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
                    return true;
                }
                false
            }
            Color::White => {
                let white_king = board
                    .get_bitboard(Color::White, PieceType::King)
                    .first_position()
                    .unwrap();
                let attackers = self.get_attackers(board, white_king, Color::Black);
                if attackers > 0 {
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
                    Bitboard::knight_targets(target, Bitboard::empty())
                        & board.get_bitboard(Black, Knight)
                }
                (Black, Bishop) => {
                    Bitboard::white_bishop_targets(
                        target,
                        board.white_occupancy(),
                        board.black_occupancy(),
                    ) & board.get_bitboard(Black, Bishop)
                }
                (Black, Rook) => {
                    Bitboard::white_rook_targets(
                        target,
                        board.white_occupancy(),
                        board.black_occupancy(),
                    ) & board.get_bitboard(Black, Rook)
                }
                (Black, Queen) => {
                    Bitboard::white_queen_targets(
                        target,
                        board.white_occupancy(),
                        board.black_occupancy(),
                    ) & board.get_bitboard(Black, Queen)
                }
                (Black, King) => {
                    Bitboard::white_king_targets(target, board.white_occupancy())
                        & board.get_bitboard(Black, King)
                }
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
                    Bitboard::knight_targets(target, Bitboard::empty())
                        & board.get_bitboard(White, Knight)
                }
                (White, Bishop) => {
                    Bitboard::black_bishop_targets(
                        target,
                        board.white_occupancy(),
                        board.black_occupancy(),
                    ) & board.get_bitboard(White, Bishop)
                }
                (White, Rook) => {
                    Bitboard::black_rook_targets(
                        target,
                        board.white_occupancy(),
                        board.black_occupancy(),
                    ) & board.get_bitboard(White, Rook)
                }
                (White, Queen) => {
                    Bitboard::black_queen_targets(
                        target,
                        board.white_occupancy(),
                        board.black_occupancy(),
                    ) & board.get_bitboard(White, Queen)
                }
                (White, King) => {
                    Bitboard::black_king_targets(target, board.black_occupancy())
                        & board.get_bitboard(White, King)
                }
            }
        }
        attacker_bb
    }

    fn evaluate_legal_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> HashSet<ChessMove> {
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
        HashSet::new()
    }

    fn evaluate_legal_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> HashSet<ChessMove> {
        if from.rank() == Rank::One || from.rank() == Rank::Eight {
            // TODO: mark this as an error somehow?
            // There should never be a pawn on the first or eighth ranks.
            return HashSet::new();
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
    ) -> HashSet<ChessMove> {
        let mut legal_moves = HashSet::with_capacity(10);
        if from.rank() == Rank::Seven {
            // from here it's only possible to promote

            // check position in front
            let up = from.up().unwrap();
            if !board.has_piece_at(up) {
                legal_moves.extend(ChessMove::promotion_moves(from, up));
            }
            if let Some(up_left) = from.up_left() {
                if board.has_piece_of_color_at(Color::Black, up_left) {
                    legal_moves.extend(ChessMove::promotion_moves(from, up_left));
                }
            }
            if let Some(up_right) = from.up_right() {
                if board.has_piece_of_color_at(Color::Black, up_right) {
                    legal_moves.extend(ChessMove::promotion_moves(from, up_right));
                }
            }
        } else {
            let targets = Bitboard::white_pawn_targets(
                from,
                board.white_occupancy(),
                board.black_occupancy(),
            );
            for to in targets.positions() {
                legal_moves.insert(ChessMove::Regular { from, to });
            }

            // en passant
            if let t @ Some(en_passant_target) = self.white_en_passant_target {
                if from.rank() == Rank::Five && (from.up_left() == t || from.up_right() == t) {
                    legal_moves.insert(ChessMove::EnPassant {
                        from,
                        to: en_passant_target,
                        taken_original_index: Position::new(en_passant_target.file(), Rank::Seven),
                        taken_index: Position::new(en_passant_target.file(), Rank::Five),
                    });
                }
            }
        }
        legal_moves
    }

    fn evaluate_legal_black_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> HashSet<ChessMove> {
        let mut legal_moves = HashSet::with_capacity(10);
        if from.rank() == Rank::Two {
            // from here it's only possible to promote

            // check position in front
            let down = from.down().unwrap();
            if !board.has_piece_at(down) {
                legal_moves.extend(ChessMove::promotion_moves(from, down));
            }
            if let Some(down_left) = from.down_left() {
                if board.has_piece_of_color_at(Color::White, down_left) {
                    legal_moves.extend(ChessMove::promotion_moves(from, down_left));
                }
            }
            if let Some(down_right) = from.down_right() {
                if board.has_piece_of_color_at(Color::White, down_right) {
                    legal_moves.extend(ChessMove::promotion_moves(from, down_right));
                }
            }
        } else {
            let targets =
                Bitboard::black_pawn_targets(from, board.white_occupancy(), board.black_occupancy());
            for to in targets.positions() {
                legal_moves.insert(ChessMove::Regular { from, to });
            }

            // en passant
            if let t @ Some(en_passant_target) = self.black_en_passant_target {
                if from.rank() == Rank::Four && (from.down_left() == t || from.down_right() == t) {
                    legal_moves.insert(ChessMove::EnPassant {
                        from,
                        to: en_passant_target,
                        taken_original_index: Position::new(en_passant_target.file(), Rank::Two),
                        taken_index: Position::new(en_passant_target.file(), Rank::Four),
                    });
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
    ) -> HashSet<ChessMove> {
        use bitboard64::prelude::*;
        let targets = match player {
            Color::Black => Bitboard::black_king_targets(from, board.black_occupancy()),
            Color::White => Bitboard::white_king_targets(from, board.white_occupancy()),
        };
        let mut legal_moves: HashSet<ChessMove> = targets
            .positions()
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        match (player, from) {
            (Color::Black, E8) => {
                if let Some(mut castle_moves) = self.evaluate_black_castle(board) {
                    legal_moves.extend(castle_moves);
                }
            }
            (Color::White, E1) => {
                if let Some(mut castle_moves) = self.evaluate_white_castle(board) {
                    legal_moves.extend(castle_moves);
                }
            }
            _ => {}
        }
        legal_moves
    }

    fn evaluate_white_castle(&self, board: &ChessBoard) -> Option<Vec<ChessMove>> {
        use bitboard64::prelude::*;
        let mut moves = Vec::with_capacity(2);

        // check short castle
        if self.castling_rights.white_kingside()
            && !board.has_piece_at(F1)
            && !board.has_piece_at(G1)
            && !self.is_under_attack(board, F1, Color::Black)
            && !self.is_under_attack(board, G1, Color::Black)
        {
            moves.push(ChessMove::Castle {
                rook_from: H1,
                rook_to: F1,
                king_from: E1,
                king_to: G1,
            });
        }

        // check long castle
        if self.castling_rights.white_queenside()
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
        use bitboard64::prelude::*;

        let mut moves = Vec::with_capacity(2);

        // check short castle
        if self.castling_rights.black_kingside()
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
        if self.castling_rights.black_queenside()
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
    ) -> HashSet<ChessMove> {
        let targets = Bitboard::knight_targets(
            from,
            match player {
                Color::Black => board.black_occupancy(),
                Color::White => board.white_occupancy(),
            },
        ) & !board.get_occupancy_for_color(player);

        let mut legal_moves = HashSet::with_capacity(8);
        for to in targets.positions() {
            legal_moves.insert(ChessMove::Regular { from, to });
        }
        legal_moves
    }

    fn evaluate_legal_bishop_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> HashSet<ChessMove> {
        let targets = match player {
            Color::Black => Bitboard::black_bishop_targets(
                from,
                board.white_occupancy(),
                board.black_occupancy(),
            ),
            Color::White => Bitboard::white_bishop_targets(
                from,
                board.white_occupancy(),
                board.black_occupancy(),
            ),
        };
        let mut legal_moves = HashSet::with_capacity(16);
        for to in targets.positions() {
            legal_moves.insert(ChessMove::Regular { from, to });
        }
        legal_moves
    }

    fn evaluate_legal_rook_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> HashSet<ChessMove> {
        let targets = match player {
            Color::Black => {
                Bitboard::black_rook_targets(from, board.white_occupancy(), board.black_occupancy())
            }
            Color::White => {
                Bitboard::white_rook_targets(from, board.white_occupancy(), board.black_occupancy())
            }
        };

        let mut legal_moves = HashSet::with_capacity(16);
        for to in targets.positions() {
            legal_moves.insert(ChessMove::Regular { from, to });
        }
        legal_moves
    }

    fn evaluate_legal_queen_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> HashSet<ChessMove> {
        let targets = match player {
            Color::Black => Bitboard::black_queen_targets(
                from,
                board.white_occupancy(),
                board.black_occupancy(),
            ),
            Color::White => Bitboard::white_queen_targets(
                from,
                board.white_occupancy(),
                board.black_occupancy(),
            ),
        };
        let mut legal_moves = HashSet::with_capacity(16);
        for to in targets.positions() {
            legal_moves.insert(ChessMove::Regular { from, to });
        }
        legal_moves
    }
}

impl Default for MoveManager {
    fn default() -> Self {
        Self {
            board_history: vec![],
            move_history: vec![],
            legal_moves: HashSet::with_capacity(30),
            white_en_passant_target: None,
            black_en_passant_target: None,
            castling_rights: CastlingRights::default(),
            half_moves: 0,
            full_moves: 1,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl CastlingRights {
    pub(crate) fn new(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> Self {
        Self {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }

    pub(crate) fn white_kingside(&self) -> bool {
        self.white_kingside
    }

    pub(crate) fn white_kingside_mut(&mut self) -> &mut bool {
        &mut self.white_kingside
    }

    pub(crate) fn white_queenside(&self) -> bool {
        self.white_queenside
    }

    pub(crate) fn white_queenside_mut(&mut self) -> &mut bool {
        &mut self.white_queenside
    }

    pub(crate) fn black_kingside(&self) -> bool {
        self.black_kingside
    }

    pub(crate) fn black_kingside_mut(&mut self) -> &mut bool {
        &mut self.black_kingside
    }

    pub(crate) fn black_queenside(&self) -> bool {
        self.black_queenside
    }

    pub(crate) fn black_queenside_mut(&mut self) -> &mut bool {
        &mut self.black_queenside
    }

    pub(crate) fn as_fen_string(&self) -> String {
        if (
            self.white_kingside,
            self.white_queenside,
            self.black_kingside,
            self.black_queenside,
        ) == (false, false, false, false)
        {
            return "-".to_string();
        } else {
            let mut buf = String::with_capacity(4);
            if self.white_kingside {
                buf.push('K');
            }
            if self.white_queenside {
                buf.push('Q');
            }
            if self.black_kingside {
                buf.push('k');
            }
            if self.black_queenside {
                buf.push('q');
            }
            buf
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl FromStr for CastlingRights {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut wk, mut wq, mut bk, mut bq) = (false, false, false, false);

        if s != "-" {
            if s.is_empty() {
                return Err("invalid castling rights".to_string());
            }
            for c in s.chars() {
                match c {
                    'K' => {
                        if !wk {
                            wk = true;
                        } else {
                            return Err("invalid castling rights".to_string());
                        }
                    }
                    'Q' => {
                        if !wq {
                            wq = true;
                        } else {
                            return Err("invalid castling rights".to_string());
                        }
                    }
                    'k' => {
                        if !bk {
                            bk = true;
                        } else {
                            return Err("invalid castling rights".to_string());
                        }
                    }
                    'q' => {
                        if !bq {
                            bq = true;
                        } else {
                            return Err("invalid castling rights".to_string());
                        }
                    }
                    _ => return Err("invalid castling rights".to_string()),
                }
            }
        } else {
            (wk, wq, bk, bq) = (true, true, true, true)
        }

        Ok(CastlingRights::new(wk, wq, bk, bq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitboard64::prelude::*;
    use Color::*;

    #[test]
    fn legal_moves() {
        let board = ChessBoard::default();
        let mut manager = MoveManager::default();
        manager.evaluate_legal_moves(&board, White);

        dbg!(&manager);

        let legal_moves: HashSet<_> = manager
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
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn bishop_moves() {
        let board = ChessBoard::default();
        let manager = MoveManager::default();

        let bishop_moves_from_f4: HashSet<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, F4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            bishop_moves_from_f4,
            [E3, G3, E5, G5, D6, H6, C7].iter().copied().collect()
        );

        let bishop_moves_from_c1: HashSet<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, C1, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_c1, HashSet::new());
    }

    #[test]
    fn rook_moves() {
        let board = ChessBoard::default();
        let manager = MoveManager::default();

        let rook_moves_from_c5: HashSet<Position> = manager
            .evaluate_legal_rook_moves_from(&board, C5, Black)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            rook_moves_from_c5,
            [C2, C3, C4, A5, B5, D5, E5, F5, G5, H5, C6]
                .iter()
                .copied()
                .collect()
        );
    }

    #[test]
    fn knight_moves() {
        let board = ChessBoard::default();
        let manager = MoveManager::default();

        let knight_moves_from_g4: HashSet<Position> = manager
            .evaluate_legal_knight_moves_from(&board, G4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            knight_moves_from_g4,
            [E3, E5, F6, H6].iter().copied().collect()
        );

        let knight_moves_from_d4: HashSet<Position> = manager
            .evaluate_legal_knight_moves_from(&board, D4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            knight_moves_from_d4,
            [B3, F3, B5, F5, C6, E6].iter().copied().collect()
        );
    }

    #[test]
    fn queen_moves() {
        let board = ChessBoard::default();
        let manager = MoveManager::default();

        let queen_moves_from_a4: HashSet<Position> = manager
            .evaluate_legal_queen_moves_from(&board, A4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            queen_moves_from_a4,
            [A3, B3, B4, C4, D4, E4, F4, G4, H4, A5, B5, A6, C6, A7, D7]
                .iter()
                .copied()
                .collect()
        );
    }

    #[test]
    /// Remove all pawns from a chessboard and check legal moves for white
    fn moves_test_1() {
        use ChessMove::*;
        let mut board = ChessBoard::default();
        for pos in [
            A2, B2, C2, D2, E2, F2, G2, H2, A7, B7, C7, D7, E7, F7, G7, H7,
        ] {
            board.take_piece(pos).unwrap();
        }

        dbg!(board);

        let mut move_manager = MoveManager::default();
        move_manager.evaluate_legal_moves(&board, White);
        let moves = move_manager.get_legal_moves();
        let expected: HashSet<_> = [
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
        ]
        .iter()
        .copied()
        .collect();

        assert_eq!(moves.len(), expected.len());
        assert_eq!(moves, &expected);
    }
}
