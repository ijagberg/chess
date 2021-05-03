use crate::consts::*;
use crate::Position;
use std::{fmt::Debug, ops::BitOr};

pub struct ChessBoard {
    white_king: Bitboard,
    black_king: Bitboard,
    white_queens: Bitboard,
    black_queens: Bitboard,
    white_rooks: Bitboard,
    black_rooks: Bitboard,
    white_knights: Bitboard,
    black_knights: Bitboard,
    white_bishops: Bitboard,
    black_bishops: Bitboard,
    white_pawns: Bitboard,
    black_pawns: Bitboard,
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

        Self {
            white_king: Bitboard::white_king(),
            black_king: Bitboard::black_king(),
            white_queens: Bitboard::white_queens(),
            black_queens: Bitboard::black_queens(),
            white_rooks: Bitboard::white_rooks(),
            black_rooks: Bitboard::black_rooks(),
            white_knights: Bitboard::white_knights(),
            black_knights: Bitboard::black_knights(),
            white_bishops: Bitboard::white_bishops(),
            black_bishops: Bitboard::black_bishops(),
            white_pawns: Bitboard::white_pawns(),
            black_pawns: Bitboard::black_pawns(),
            file_clear,
            file_mask,
            rank_clear,
            rank_mask,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Bitboard {
    data: u64,
}

impl Bitboard {
    fn new() -> Self {
        Self::new_with_data(0)
    }

    pub(crate) fn all_pieces() -> Self {
        Self::all_white() | Self::all_black()
    }

    pub(crate) fn all_white() -> Self {
        Self::white_pawns()
            | Self::white_bishops()
            | Self::white_knights()
            | Self::white_rooks()
            | Self::white_queens()
            | Self::white_king()
    }

    pub(crate) fn all_black() -> Self {
        Self::black_pawns()
            | Self::black_bishops()
            | Self::black_knights()
            | Self::black_rooks()
            | Self::black_queens()
            | Self::black_king()
    }

    pub(crate) fn white_queens() -> Self {
        let data = 0b00001000_u64 << 0;
        Self::new_with_data(data)
    }

    pub(crate) fn black_queens() -> Self {
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
        Self::new().with_one(E1)
    }

    pub(crate) fn black_king() -> Self {
        Self::new().with_one(E8)
    }

    pub(crate) fn all_kings() -> Self {
        Self::white_king() | Self::black_king()
    }

    fn new_with_data(data: u64) -> Self {
        Self { data }
    }

    fn with_one(mut self, pos: Position) -> Self {
        self.set_bit(pos);
        self
    }

    pub fn set_bit(&mut self, pos: Position) {
        let bit_index = Self::get_bit_index(pos);
        dbg!(bit_index);

        let bit_to_set = 1_u64 << bit_index;
        self.data = self.data | bit_to_set;
    }

    fn get_bit_index(pos: Position) -> u32 {
        let file = u32::from(pos.file()) - 1;
        let rank = u32::from(pos.rank()) - 1;

        let nth_bit = (8 * rank) + file;
        nth_bit
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = self.data | rhs.data;
        Self::new_with_data(result)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary = format!("{:064b}", self.data);
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
        assert_eq!(white_rooks.data, 129);

        let black_rooks = Bitboard::black_rooks();
        println!("black:\n{:?}", black_rooks);
        assert_eq!(black_rooks.data, 9295429630892703744);
    }

    #[test]
    fn pawn_bitboards() {
        let white_pawns = Bitboard::white_pawns();
        println!("white pawns:\n{:?}", white_pawns);
        assert_eq!(white_pawns.data, 65280);

        let black_pawns = Bitboard::black_pawns();
        println!("black pawns:\n{:?}", black_pawns);
        assert_eq!(black_pawns.data, 71776119061217280);
    }

    #[test]
    fn bishop_bitboards() {
        let white_bishops = Bitboard::white_bishops();
        println!("white bishops:\n{:?}", white_bishops);
        assert_eq!(white_bishops.data, 36);

        let black_bishops = Bitboard::black_bishops();
        println!("black bishops:\n{:?}", black_bishops);
        assert_eq!(black_bishops.data, 2594073385365405696);
    }

    #[test]
    fn knight_bitboards() {
        let white_knights = Bitboard::white_knights();
        println!("white knights:\n{:?}", white_knights);
        assert_eq!(white_knights.data, 66);

        let black_knights = Bitboard::black_knights();
        println!("black knights:\n{:?}", black_knights);
        assert_eq!(black_knights.data, 4755801206503243776);
    }

    #[test]
    fn queen_bitboards() {
        let white_queen = Bitboard::white_queens();
        println!("white queen:\n{:?}", white_queen);
        assert_eq!(white_queen.data, 8);

        let black_queen = Bitboard::black_queens();
        println!("black queen:\n{:?}", black_queen);
        assert_eq!(black_queen.data, 576460752303423488);
    }

    #[test]
    fn union_bitboards() {
        let all_white = Bitboard::all_white();
        println!("white pieces:\n{:?}", all_white);
        assert_eq!(all_white.data, 65535);

        let all_black = Bitboard::all_black();
        println!("black pieces:\n{:?}", all_black);
        assert_eq!(all_black.data, 18446462598732840960);

        let all_pieces = Bitboard::all_pieces();
        println!("all pieces:\n{:?}", all_pieces);
        assert_eq!(all_pieces.data, 18446462598732906495);
    }

    #[test]
    fn file_masks() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.file_mask[0]);
        assert_eq!(b.file_mask[0].data, 72340172838076673);
        println!("2:\n{:?}", b.file_mask[1]);
        assert_eq!(b.file_mask[1].data, 144680345676153346);
        println!("3:\n{:?}", b.file_mask[2]);
        assert_eq!(b.file_mask[2].data, 289360691352306692);
        println!("4:\n{:?}", b.file_mask[3]);
        assert_eq!(b.file_mask[3].data, 578721382704613384);
        println!("5:\n{:?}", b.file_mask[4]);
        assert_eq!(b.file_mask[4].data, 1157442765409226768);
        println!("6:\n{:?}", b.file_mask[5]);
        assert_eq!(b.file_mask[5].data, 2314885530818453536);
        println!("7:\n{:?}", b.file_mask[6]);
        assert_eq!(b.file_mask[6].data, 4629771061636907072);
        println!("8:\n{:?}", b.file_mask[7]);
        assert_eq!(b.file_mask[7].data, 9259542123273814144);
    }

    #[test]
    fn file_clears() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.file_clear[0]);
        assert_eq!(b.file_clear[0].data, 18374403900871474942);
        println!("2:\n{:?}", b.file_clear[1]);
        assert_eq!(b.file_clear[1].data, 18302063728033398269);
        println!("3:\n{:?}", b.file_clear[2]);
        assert_eq!(b.file_clear[2].data, 18157383382357244923);
        println!("4:\n{:?}", b.file_clear[3]);
        assert_eq!(b.file_clear[3].data, 17868022691004938231);
        println!("5:\n{:?}", b.file_clear[4]);
        assert_eq!(b.file_clear[4].data, 17289301308300324847);
        println!("6:\n{:?}", b.file_clear[5]);
        assert_eq!(b.file_clear[5].data, 16131858542891098079);
        println!("7:\n{:?}", b.file_clear[6]);
        assert_eq!(b.file_clear[6].data, 13816973012072644543);
        println!("8:\n{:?}", b.file_clear[7]);
        assert_eq!(b.file_clear[7].data, 9187201950435737471);
    }

    #[test]
    fn rank_masks() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.rank_mask[0]);
        assert_eq!(b.rank_mask[0].data, 255);
        println!("2:\n{:?}", b.rank_mask[1]);
        assert_eq!(b.rank_mask[1].data, 65280);
        println!("3:\n{:?}", b.rank_mask[2]);
        assert_eq!(b.rank_mask[2].data, 16711680);
        println!("4:\n{:?}", b.rank_mask[3]);
        assert_eq!(b.rank_mask[3].data, 4278190080);
        println!("5:\n{:?}", b.rank_mask[4]);
        assert_eq!(b.rank_mask[4].data, 1095216660480);
        println!("6:\n{:?}", b.rank_mask[5]);
        assert_eq!(b.rank_mask[5].data, 280375465082880);
        println!("7:\n{:?}", b.rank_mask[6]);
        assert_eq!(b.rank_mask[6].data, 71776119061217280);
        println!("8:\n{:?}", b.rank_mask[7]);
        assert_eq!(b.rank_mask[7].data, 18374686479671623680);
    }

    #[test]
    fn rank_clears() {
        let b = ChessBoard::new();
        println!("1:\n{:?}", b.rank_clear[0]);
        assert_eq!(b.rank_clear[0].data, 18446744073709551360);
        println!("2:\n{:?}", b.rank_clear[1]);
        assert_eq!(b.rank_clear[1].data, 18446744073709486335);
        println!("3:\n{:?}", b.rank_clear[2]);
        assert_eq!(b.rank_clear[2].data, 18446744073692839935);
        println!("4:\n{:?}", b.rank_clear[3]);
        assert_eq!(b.rank_clear[3].data, 18446744069431361535);
        println!("5:\n{:?}", b.rank_clear[4]);
        assert_eq!(b.rank_clear[4].data, 18446742978492891135);
        println!("6:\n{:?}", b.rank_clear[5]);
        assert_eq!(b.rank_clear[5].data, 18446463698244468735);
        println!("7:\n{:?}", b.rank_clear[6]);
        assert_eq!(b.rank_clear[6].data, 18374967954648334335);
        println!("8:\n{:?}", b.rank_clear[7]);
        assert_eq!(b.rank_clear[7].data, 72057594037927935);
    }
}
