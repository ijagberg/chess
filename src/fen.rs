use crate::{chess_board::ChessBoard, chess_move::CastlingRights, Color};
use bitboard64::prelude::*;
use std::{convert::TryFrom, fmt::Display, str::FromStr};

pub(crate) struct Fen {
    board: ChessBoard,
    current_player: Color,
    castling_rights: CastlingRights,
    white_en_passant_target: Option<Position>,
    black_en_passant_target: Option<Position>,
    halfmoves: u32,
    fullmoves: u32,
}

impl Fen {
    pub(crate) fn new(
        board: ChessBoard,
        current_player: Color,
        castling_rights: CastlingRights,
        white_en_passant_target: Option<Position>,
        black_en_passant_target: Option<Position>,
        halfmoves: u32,
        fullmoves: u32,
    ) -> Self {
        Self {
            board,
            current_player,
            castling_rights,
            white_en_passant_target,
            black_en_passant_target,
            halfmoves,
            fullmoves,
        }
    }

    pub(crate) fn board(&self) -> ChessBoard {
        self.board
    }

    pub(crate) fn current_player(&self) -> Color {
        self.current_player
    }

    pub(crate) fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub(crate) fn halfmoves(&self) -> u32 {
        self.halfmoves
    }

    pub(crate) fn fullmoves(&self) -> u32 {
        self.fullmoves
    }

    pub(crate) fn white_en_passant_target(&self) -> Option<Position> {
        self.white_en_passant_target
    }

    pub(crate) fn black_en_passant_target(&self) -> Option<Position> {
        self.black_en_passant_target
    }
}

impl FromStr for Fen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();

        if parts.len() != 6 {
            return Err(format!("fen string must contain exactly 6 parts"));
        }

        let board = board_from_fen_part_0(parts[0])?;

        let current_player = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            e => return Err(format!("invalid player '{}'", e)),
        };

        let castling_rights = CastlingRights::from_str(parts[2])?;

        let (black_en_passant_target, white_en_passant_target) = if parts[3] == "-" {
            (None, None)
        } else {
            if let Ok(pos) =
                Position::from_str(parts[3]).map_err(|_| "invalid en passant target".to_string())
            {
                if pos.rank() == Rank::Seven {
                    (None, Some(pos))
                } else if pos.rank() == Rank::Two {
                    (Some(pos), None)
                } else {
                    return Err("invalid en passant target".to_string());
                }
            } else {
                return Err("invalid en passant target".to_string());
            }
        };

        let halfmoves: u32 = parts[4]
            .parse()
            .map_err(|_| "invalid half moves".to_string())?;
        let fullmoves: u32 = parts[5]
            .parse()
            .map_err(|_| "invalid full moves".to_string())?;
        Ok(Self::new(
            board,
            current_player,
            castling_rights,
            white_en_passant_target,
            black_en_passant_target,
            halfmoves,
            fullmoves,
        ))
    }
}

fn board_from_fen_part_0(part0: &str) -> Result<ChessBoard, String> {
    let empty = Bitboard::empty();
    let (
        mut white_kings,
        mut black_kings,
        mut all_kings,
        mut white_queens,
        mut black_queens,
        mut all_queens,
        mut white_rooks,
        mut black_rooks,
        mut all_rooks,
        mut white_knights,
        mut black_knights,
        mut all_knights,
        mut white_bishops,
        mut black_bishops,
        mut all_bishops,
        mut white_pawns,
        mut black_pawns,
        mut all_pawns,
        mut white_pieces,
        mut black_pieces,
        mut all_pieces,
    ) = (
        empty, empty, empty, empty, empty, empty, empty, empty, empty, empty, empty, empty, empty,
        empty, empty, empty, empty, empty, empty, empty, empty,
    );

    let rows: Vec<_> = part0.split("/").collect();
    if rows.len() != 8 {
        return Err("invalid board".to_string());
    }

    for (row, rank) in rows.iter().zip(Rank::Eight.walk_down()) {
        // check that the sum of the content is 8
        if row
            .chars()
            .map(|c| {
                if let Some(digit) = c.to_digit(10) {
                    digit
                } else {
                    1
                }
            })
            .sum::<u32>()
            != 8
        {
            return Err("invalid board".to_string());
        }

        let mut current_file = File::A;
        for c in row.chars() {
            let pos = Position::new(current_file, rank);
            let bb = Bitboard::with_one(pos);
            if let Some(digit) = c.to_digit(10) {
                // c empty squares starting at `current_file`
                let digit = digit as i32;
                if digit + i32::from(u8::from(current_file)) == 8 {
                    continue;
                } else {
                    current_file = add_file_offset(current_file, digit as i32).unwrap();
                }
            } else {
                if current_file != File::H {
                    current_file = current_file.right().unwrap();
                }
                match c {
                    'p' => {
                        black_pawns |= bb;
                        all_pawns |= bb;
                        black_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'n' => {
                        black_knights |= bb;
                        all_knights |= bb;
                        black_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'b' => {
                        black_bishops |= bb;
                        all_bishops |= bb;
                        black_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'r' => {
                        black_rooks |= bb;
                        all_rooks |= bb;
                        black_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'q' => {
                        black_queens |= bb;
                        all_queens |= bb;
                        black_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'k' => {
                        black_kings |= bb;
                        all_kings |= bb;
                        black_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'P' => {
                        white_pawns |= bb;
                        all_pawns |= bb;
                        white_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'N' => {
                        white_knights |= bb;
                        all_knights |= bb;
                        white_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'B' => {
                        white_bishops |= bb;
                        all_bishops |= bb;
                        white_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'R' => {
                        white_rooks |= bb;
                        all_rooks |= bb;
                        white_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'Q' => {
                        white_queens |= bb;
                        all_queens |= bb;
                        white_pieces |= bb;
                        all_pieces |= bb;
                    }
                    'K' => {
                        white_kings |= bb;
                        all_kings |= bb;
                        white_pieces |= bb;
                        all_pieces |= bb;
                    }
                    e => return Err(format!("invalid piece char '{}'", e)),
                }
            }
        }
    }
    Ok(ChessBoard::new(
        white_kings,
        black_kings,
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
    ))
}

impl Display for Fen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                self.board().to_fen_string(),
                self.current_player().fen_char().to_string(),
                self.castling_rights().as_fen_string(),
                self.white_en_passant_target()
                    .or(self.black_en_passant_target())
                    .map(|pos| pos.to_string())
                    .unwrap_or("-".to_string()),
                self.halfmoves().to_string(),
                self.fullmoves().to_string(),
            ]
            .join(" ")
        )
    }
}

pub(crate) fn add_file_offset(file: File, offset: i32) -> Option<File> {
    let v = (u8::from(file)) as i32 + offset;
    File::try_from(u8::try_from(v).ok()?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{piece::PieceType, Piece};
    use bitboard64::prelude::*;

    #[test]
    fn fen_test() {
        use PieceType::*;

        let fen = Fen::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0").unwrap();
        let board = fen.board;
        assert_eq!(board, ChessBoard::default());

        let fen = Fen::from_str("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 w - - 0 0").unwrap();
        let board = fen.board;
        for pos in INCREASING_A1_B1 {
            let piece = board.get_piece(pos);
            if [A4, B5, D6, E5, F4, H5].contains(&pos) {
                assert_eq!(
                    piece.unwrap(),
                    Piece::black(Pawn),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else if [A3, B4, D5, E4, F3, H4].contains(&pos) {
                assert_eq!(
                    piece.unwrap(),
                    Piece::white(Pawn),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else if pos == F7 {
                assert_eq!(
                    piece.unwrap(),
                    Piece::black(King),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else if pos == H3 {
                assert_eq!(
                    piece.unwrap(),
                    Piece::white(King),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else {
                assert!(piece.is_none(), "pos: {}, piece: {:?}", pos, piece);
            }
        }
        // assert_eq!(board, ChessBoard::new());
    }
}
