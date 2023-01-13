use crate::consts::*;
use crate::piece::PieceType;
use crate::Color;
use crate::Piece;
use crate::Position;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOrAssign;
use std::ops::Deref;
use std::{fmt::Debug, ops::BitOr};

pub struct ChessBoard {
    white_king: Bitboard,
    black_king: Bitboard,
    all_kings: Bitboard,
    white_queens: Bitboard,
    black_queens: Bitboard,
    all_queens: Bitboard,
    white_rooks: Bitboard,
    black_rooks: Bitboard,
    all_rooks: Bitboard,
    white_knights: Bitboard,
    black_knights: Bitboard,
    all_knights: Bitboard,
    white_bishops: Bitboard,
    black_bishops: Bitboard,
    all_bishops: Bitboard,
    white_pawns: Bitboard,
    black_pawns: Bitboard,
    all_pawns: Bitboard,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    all_pieces: Bitboard,
    file_clear: [Bitboard; 8],
    file_mask: [Bitboard; 8],
    rank_clear: [Bitboard; 8],
    rank_mask: [Bitboard; 8],
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut file_mask = [Bitboard::new(); 8];
        let mut file_clear = [Bitboard::new(); 8];
        for shift in 0..8 {
            let mask =
                0b0000000100000001000000010000000100000001000000010000000100000001_u64 << shift;
            let clear = !mask;
            file_mask[shift] = Bitboard::new_with_data(mask);
            file_clear[shift] = Bitboard::new_with_data(clear);
        }

        let mut rank_mask = [Bitboard::new(); 8];
        let mut rank_clear = [Bitboard::new(); 8];
        for shift in 0..8 {
            let mask = 0b11111111_u64 << (8 * shift);
            let clear = !mask;
            rank_mask[shift] = Bitboard::new_with_data(mask);
            rank_clear[shift] = Bitboard::new_with_data(clear);
        }

        let white_king = Bitboard::white_king();
        let black_king = Bitboard::black_king();
        let all_kings = white_king | black_king;
        let white_queens = Bitboard::white_queen();
        let black_queens = Bitboard::black_queen();
        let all_queens = white_queens | black_queens;
        let white_rooks = Bitboard::white_rooks();
        let black_rooks = Bitboard::black_rooks();
        let all_rooks = white_rooks | black_rooks;
        let white_knights = Bitboard::white_knights();
        let black_knights = Bitboard::black_knights();
        let all_knights = white_knights | black_knights;
        let white_bishops = Bitboard::white_bishops();
        let black_bishops = Bitboard::black_bishops();
        let all_bishops = white_bishops | black_bishops;
        let white_pawns = Bitboard::white_pawns();
        let black_pawns = Bitboard::black_pawns();
        let all_pawns = white_pawns | black_pawns;
        let white_pieces =
            white_king | white_queens | white_rooks | white_knights | white_bishops | white_pawns;
        let black_pieces =
            black_king | black_queens | black_rooks | black_knights | black_bishops | black_pawns;
        let all_pieces = white_pieces | black_pieces;

        Self {
            white_king,
            black_king,
            all_kings,
            white_queens,
            black_queens,
            all_queens,
            white_rooks,
            black_rooks,
            all_rooks,
            white_knights,
            black_knights,
            all_knights,
            white_bishops,
            black_bishops,
            all_bishops,
            white_pawns,
            black_pawns,
            all_pawns,
            white_pieces,
            black_pieces,
            all_pieces,
            file_clear,
            file_mask,
            rank_clear,
            rank_mask,
        }
    }

    fn get_color_of_pos(&self, pos: Position) -> Option<Color> {
        let bit_index = Bitboard::new().with_one(pos);

        let white_pieces = self.white_pieces & bit_index;
        if white_pieces.is_nonzero() {
            return Some(Color::White);
        }

        let black_pieces = self.black_pieces & bit_index;
        if black_pieces.is_nonzero() {
            return Some(Color::Black);
        }

        None
    }

    fn get_kind_of_pos(&self, pos: Position) -> Option<PieceType> {
        let bit_index = Bitboard::new().with_one(pos);

        if (self.all_kings & bit_index).is_nonzero() {
            return Some(PieceType::King);
        }

        if (self.all_queens & bit_index).is_nonzero() {
            return Some(PieceType::Queen);
        }

        if (self.all_rooks & bit_index).is_nonzero() {
            return Some(PieceType::Rook);
        }

        if (self.all_knights & bit_index).is_nonzero() {
            return Some(PieceType::Knight);
        }

        if (self.all_bishops & bit_index).is_nonzero() {
            return Some(PieceType::Bishop);
        }

        if (self.all_pawns & bit_index).is_nonzero() {
            return Some(PieceType::Pawn);
        }

        None
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        let bit_index = Bitboard::new().with_one(pos);

        let color = self.get_color_of_pos(pos)?;
        let kind = self.get_kind_of_pos(pos)?;

        Some(Piece::new(color, kind))
    }
}

#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    fn new() -> Self {
        Self::new_with_data(0)
    }

    fn new_with_data(data: u64) -> Self {
        Self(data)
    }

    fn with_one(mut self, pos: Position) -> Self {
        self.set_bit(pos);
        self
    }

    pub(crate) fn is_nonzero(&self) -> bool {
        !self.is_zero()
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.data() == 0
    }

    pub(crate) fn data(&self) -> u64 {
        self.0
    }

    pub(crate) fn all_pieces() -> Self {
        Self::all_white() | Self::all_black()
    }

    pub(crate) fn all_white() -> Self {
        Self::white_pawns()
            | Self::white_bishops()
            | Self::white_knights()
            | Self::white_rooks()
            | Self::white_queen()
            | Self::white_king()
    }

    pub(crate) fn all_black() -> Self {
        Self::black_pawns()
            | Self::black_bishops()
            | Self::black_knights()
            | Self::black_rooks()
            | Self::black_queen()
            | Self::black_king()
    }

    pub(crate) fn white_queen() -> Self {
        let data = 0b00001000_u64 << 0;
        Self::new_with_data(data)
    }

    pub(crate) fn black_queen() -> Self {
        let data = 0b00001000_u64 << 56;
        Self::new_with_data(data)
    }

    pub(crate) fn white_knights() -> Self {
        let data = 0b01000010_u64 << 0;
        Self::new_with_data(data)
    }

    pub(crate) fn black_knights() -> Self {
        let data = 0b01000010_u64 << 56;
        Self::new_with_data(data)
    }

    pub(crate) fn white_bishops() -> Self {
        let data = 0b00100100_u64 << 0;
        Self::new_with_data(data)
    }

    pub(crate) fn black_bishops() -> Self {
        let data = 0b00100100_u64 << 56;
        Self::new_with_data(data)
    }

    pub(crate) fn white_rooks() -> Self {
        let data = 0b10000001_u64 << 0;
        Self::new_with_data(data)
    }

    pub(crate) fn black_rooks() -> Self {
        let data = 0b10000001_u64 << 56;
        Self::new_with_data(data)
    }

    pub(crate) fn white_pawns() -> Self {
        let data = 0b11111111_u64 << 8;
        Self::new_with_data(data)
    }

    pub(crate) fn black_pawns() -> Self {
        let data = 0b11111111_u64 << 48;
        Self::new_with_data(data)
    }

    pub(crate) fn white_king() -> Self {
        println!("white king at E1");
        Self::new().with_one(E1)
    }

    pub(crate) fn black_king() -> Self {
        println!("black king at E8");
        Self::new().with_one(E8)
    }

    pub(crate) fn all_kings() -> Self {
        Self::white_king() | Self::black_king()
    }

    pub fn set_bit(&mut self, pos: Position) {
        let bit_index = Self::get_bit_index(pos);
        dbg!(bit_index);

        let bit_to_set = 1_u64 << bit_index;
        self.0 = self.data() | bit_to_set;
    }

    fn get_bit_index(pos: Position) -> u64 {
        let file = u64::from(pos.file()) - 1;
        let rank = u64::from(pos.rank()) - 1;

        let nth_bit = (8 * rank) + file;
        nth_bit
    }
}

impl AsRef<u64> for Bitboard {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = self.data() | rhs.data();
        Self::new_with_data(result)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.data() & rhs.data())
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary = format!("{:064b}", self.data());
        let chars: Vec<char> = binary.chars().collect();

        let mut output = Vec::new();
        let mut chunks = chars.chunks(8);
        for chunk in chunks {
            let line = chunk.iter().rev().collect::<String>();
            output.push(line);
        }

        write!(f, "{}", output.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;

    #[test]
    fn rook_bitboards() {
        let white_rooks = Bitboard::white_rooks();
        println!("white_rooks:\n{:?}", white_rooks);
        assert_eq!(white_rooks.data(), 129);

        let black_rooks = Bitboard::black_rooks();
        println!("black:\n{:?}", black_rooks);
        assert_eq!(black_rooks.data(), 9295429630892703744);
    }

    #[test]
    fn pawn_bitboards() {
        let white_pawns = Bitboard::white_pawns();
        println!("white pawns:\n{:?}", white_pawns);
        assert_eq!(white_pawns.data(), 65280);

        let black_pawns = Bitboard::black_pawns();
        println!("black pawns:\n{:?}", black_pawns);
        assert_eq!(black_pawns.data(), 71776119061217280);
    }

    #[test]
    fn bishop_bitboards() {
        let white_bishops = Bitboard::white_bishops();
        println!("white bishops:\n{:?}", white_bishops);
        assert_eq!(white_bishops.data(), 36);

        let black_bishops = Bitboard::black_bishops();
        println!("black bishops:\n{:?}", black_bishops);
        assert_eq!(black_bishops.data(), 2594073385365405696);
    }

    #[test]
    fn knight_bitboards() {
        let white_knights = Bitboard::white_knights();
        println!("white knights:\n{:?}", white_knights);
        assert_eq!(white_knights.data(), 66);

        let black_knights = Bitboard::black_knights();
        println!("black knights:\n{:?}", black_knights);
        assert_eq!(black_knights.data(), 4755801206503243776);
    }

    #[test]
    fn queen_bitboards() {
        let white_queen = Bitboard::white_queen();
        println!("white queen:\n{:?}", white_queen);
        assert_eq!(white_queen.data(), 8);

        let black_queen = Bitboard::black_queen();
        println!("black queen:\n{:?}", black_queen);
        assert_eq!(black_queen.data(), 576460752303423488);
    }

    #[test]
    fn union_bitboards() {
        let all_white = Bitboard::all_white();
        println!("white pieces:\n{:?}", all_white);
        assert_eq!(all_white.data(), 65535);

        let all_black = Bitboard::all_black();
        println!("black pieces:\n{:?}", all_black);
        assert_eq!(all_black.data(), 18446462598732840960);

        let all_pieces = Bitboard::all_pieces();
        println!("all pieces:\n{:?}", all_pieces);
        assert_eq!(all_pieces.data(), 18446462598732906495);
    }

    #[test]
    fn file_masks() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.file_mask[0]);
        assert_eq!(b.file_mask[0].data(), 72340172838076673);
        println!("2:\n{:?}", b.file_mask[1]);
        assert_eq!(b.file_mask[1].data(), 144680345676153346);
        println!("3:\n{:?}", b.file_mask[2]);
        assert_eq!(b.file_mask[2].data(), 289360691352306692);
        println!("4:\n{:?}", b.file_mask[3]);
        assert_eq!(b.file_mask[3].data(), 578721382704613384);
        println!("5:\n{:?}", b.file_mask[4]);
        assert_eq!(b.file_mask[4].data(), 1157442765409226768);
        println!("6:\n{:?}", b.file_mask[5]);
        assert_eq!(b.file_mask[5].data(), 2314885530818453536);
        println!("7:\n{:?}", b.file_mask[6]);
        assert_eq!(b.file_mask[6].data(), 4629771061636907072);
        println!("8:\n{:?}", b.file_mask[7]);
        assert_eq!(b.file_mask[7].data(), 9259542123273814144);
    }

    #[test]
    fn file_clears() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.file_clear[0]);
        assert_eq!(b.file_clear[0].data(), 18374403900871474942);
        println!("2:\n{:?}", b.file_clear[1]);
        assert_eq!(b.file_clear[1].data(), 18302063728033398269);
        println!("3:\n{:?}", b.file_clear[2]);
        assert_eq!(b.file_clear[2].data(), 18157383382357244923);
        println!("4:\n{:?}", b.file_clear[3]);
        assert_eq!(b.file_clear[3].data(), 17868022691004938231);
        println!("5:\n{:?}", b.file_clear[4]);
        assert_eq!(b.file_clear[4].data(), 17289301308300324847);
        println!("6:\n{:?}", b.file_clear[5]);
        assert_eq!(b.file_clear[5].data(), 16131858542891098079);
        println!("7:\n{:?}", b.file_clear[6]);
        assert_eq!(b.file_clear[6].data(), 13816973012072644543);
        println!("8:\n{:?}", b.file_clear[7]);
        assert_eq!(b.file_clear[7].data(), 9187201950435737471);
    }

    #[test]
    fn rank_masks() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.rank_mask[0]);
        assert_eq!(b.rank_mask[0].data(), 255);
        println!("2:\n{:?}", b.rank_mask[1]);
        assert_eq!(b.rank_mask[1].data(), 65280);
        println!("3:\n{:?}", b.rank_mask[2]);
        assert_eq!(b.rank_mask[2].data(), 16711680);
        println!("4:\n{:?}", b.rank_mask[3]);
        assert_eq!(b.rank_mask[3].data(), 4278190080);
        println!("5:\n{:?}", b.rank_mask[4]);
        assert_eq!(b.rank_mask[4].data(), 1095216660480);
        println!("6:\n{:?}", b.rank_mask[5]);
        assert_eq!(b.rank_mask[5].data(), 280375465082880);
        println!("7:\n{:?}", b.rank_mask[6]);
        assert_eq!(b.rank_mask[6].data(), 71776119061217280);
        println!("8:\n{:?}", b.rank_mask[7]);
        assert_eq!(b.rank_mask[7].data(), 18374686479671623680);
    }

    #[test]
    fn rank_clears() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.rank_clear[0]);
        assert_eq!(b.rank_clear[0].data(), 18446744073709551360);
        println!("2:\n{:?}", b.rank_clear[1]);
        assert_eq!(b.rank_clear[1].data(), 18446744073709486335);
        println!("3:\n{:?}", b.rank_clear[2]);
        assert_eq!(b.rank_clear[2].data(), 18446744073692839935);
        println!("4:\n{:?}", b.rank_clear[3]);
        assert_eq!(b.rank_clear[3].data(), 18446744069431361535);
        println!("5:\n{:?}", b.rank_clear[4]);
        assert_eq!(b.rank_clear[4].data(), 18446742978492891135);
        println!("6:\n{:?}", b.rank_clear[5]);
        assert_eq!(b.rank_clear[5].data(), 18446463698244468735);
        println!("7:\n{:?}", b.rank_clear[6]);
        assert_eq!(b.rank_clear[6].data(), 18374967954648334335);
        println!("8:\n{:?}", b.rank_clear[7]);
        assert_eq!(b.rank_clear[7].data(), 72057594037927935);
    }

    #[test]
    fn get_color_of_pos_test() {
        let b = ChessBoard::new();
        assert_eq!(b.get_color_of_pos(E1), Some(Color::White));
        assert_eq!(b.get_color_of_pos(A7), Some(Color::Black));
        assert_eq!(b.get_color_of_pos(A6), None);
    }

    #[test]
    fn get_kind_of_pos_test() {
        let b = ChessBoard::new();
        assert_eq!(b.get_kind_of_pos(E1), Some(PieceType::King));
        assert_eq!(b.get_kind_of_pos(A7), Some(PieceType::Pawn));
        assert_eq!(b.get_kind_of_pos(A6), None);
    }

    #[test]
    fn get_piece_test() {
        let b = ChessBoard::new();
        assert_eq!(
            b.get_piece(E1),
            Some(Piece::new(Color::White, PieceType::King))
        );
        assert_eq!(
            b.get_piece(A7),
            Some(Piece::new(Color::Black, PieceType::Pawn))
        );
        assert_eq!(b.get_piece(A6), None);
    }
}
