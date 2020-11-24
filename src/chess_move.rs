use crate::{ChessIndex, PieceType};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ChessMove {
    Regular(RegularMove),
    Castle(CastleMove),
    Promotion(PromotionMove),
    EnPassant(EnPassantMove),
}

impl ChessMove {
    pub fn regular(from: ChessIndex, to: ChessIndex) -> ChessMove {
        ChessMove::Regular(RegularMove::new(from, to))
    }

    pub fn promotions(from: ChessIndex, to: ChessIndex) -> Vec<ChessMove> {
        vec![
            PieceType::Knight,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::Bishop,
        ]
        .into_iter()
        .map(|pt| ChessMove::promotion(from, to, pt))
        .collect()
    }

    pub fn promotion(from: ChessIndex, to: ChessIndex, promotion_piece: PieceType) -> ChessMove {
        ChessMove::Promotion(PromotionMove::new(from, to, promotion_piece))
    }

    pub fn en_passant(from: ChessIndex, to: ChessIndex, taken_pawn_idx: ChessIndex) -> ChessMove {
        ChessMove::EnPassant(EnPassantMove::new(from, to, taken_pawn_idx))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct RegularMove(ChessIndex, ChessIndex);

impl RegularMove {
    pub fn new(from: ChessIndex, to: ChessIndex) -> Self {
        Self(from, to)
    }

    pub fn from_idx(&self) -> ChessIndex {
        self.0
    }

    pub fn to_idx(&self) -> ChessIndex {
        self.1
    }
}

impl From<(ChessIndex, ChessIndex)> for RegularMove {
    fn from((from, to): (ChessIndex, ChessIndex)) -> Self {
        RegularMove::new(from, to)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct CastleMove(ChessIndex, ChessIndex, ChessIndex, ChessIndex);

impl CastleMove {
    pub fn new(
        king_from: ChessIndex,
        king_to: ChessIndex,
        rook_from: ChessIndex,
        rook_to: ChessIndex,
    ) -> CastleMove {
        CastleMove(king_from, king_to, rook_from, rook_to)
    }

    pub fn king_from(&self) -> ChessIndex {
        self.0
    }

    pub fn king_to(&self) -> ChessIndex {
        self.1
    }

    pub fn rook_from(&self) -> ChessIndex {
        self.2
    }

    pub fn rook_to(&self) -> ChessIndex {
        self.3
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct PromotionMove(ChessIndex, ChessIndex, PieceType);

impl PromotionMove {
    pub fn new(from: ChessIndex, to: ChessIndex, promotion_piece: PieceType) -> Self {
        Self(from, to, promotion_piece)
    }

    pub fn from_idx(&self) -> ChessIndex {
        self.0
    }

    pub fn to_idx(&self) -> ChessIndex {
        self.1
    }

    pub fn promotion_piece(&self) -> PieceType {
        self.2
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EnPassantMove(ChessIndex, ChessIndex, ChessIndex);

impl EnPassantMove {
    pub fn new(from: ChessIndex, to: ChessIndex, taken_pawn_idx: ChessIndex) -> Self {
        Self(from, to, taken_pawn_idx)
    }

    pub fn from_idx(&self) -> ChessIndex {
        self.0
    }

    pub fn to_idx(&self) -> ChessIndex {
        self.1
    }

    pub fn taken_pawn_idx(&self) -> ChessIndex {
        self.2
    }
}
