use crate::{board::Board, consts::*, game::Game, piece::PieceType, Color, Piece, Position, Rank};
use std::option::Option;

const KNIGHT_OFFSETS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (1, -2),
    (-1, -2),
];

const KING_OFFSETS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ChessMove {
    Regular {
        from: Position,
        to: Position,
    },
    EnPassant {
        from: Position,
        to: Position,
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub(crate) struct MoveManager {
    history: Vec<(ChessMove, Option<Piece>)>,
    legal_moves: Vec<ChessMove>,
    black_king: Position,
    white_king: Position,
}

impl MoveManager {
    pub(crate) fn new(board: &Board, player: Color) -> Self {
        let mut white_king = None;
        let mut black_king = None;
        for &pos in &INCREASING_ORDER {
            if let Some(Piece {
                color,
                kind: PieceType::King,
            }) = board[pos]
            {
                match color {
                    Color::Black => {
                        black_king = Some(pos);
                    }
                    Color::White => {
                        white_king = Some(pos);
                    }
                }
            }
        }
        let mut this = Self {
            history: vec![],
            legal_moves: vec![],
            black_king: black_king.expect("no black king on board"),
            white_king: white_king.expect("no white king on board"),
        };

        this.evaluate_legal_moves(board, player);

        this
    }

    pub(crate) fn is_legal(&self, chess_move: ChessMove) -> bool {
        self.legal_moves.contains(&chess_move)
    }

    pub(crate) fn make_move(
        &mut self,
        board: &mut Board,
        chess_move: ChessMove,
        dry_run: bool,
    ) -> Option<Piece> {
        let taken_piece;
        match chess_move {
            ChessMove::Regular { from, to } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, piece);
                if let Piece {
                    color,
                    kind: PieceType::King,
                } = piece
                {
                    match color {
                        Color::Black => {
                            self.black_king = to;
                        }
                        Color::White => {
                            self.white_king = to;
                        }
                    }
                }
                taken_piece = taken;
            }
            ChessMove::EnPassant {
                from,
                to,
                taken_index,
            } => {
                let piece = board.take_piece(from).unwrap();
                board.set_piece(to, piece);
                let taken = board.take_piece(taken_index).unwrap();
                taken_piece = Some(taken);
            }
            ChessMove::Promotion { from, to, piece } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, piece);
                taken_piece = taken;
            }
            ChessMove::Castle {
                rook_from,
                rook_to,
                king_from,
                king_to,
            } => {
                // dont forget to update king pos
                let rook = board.take_piece(rook_from).unwrap();
                board.set_piece(rook_to, rook);
                let king = board.take_piece(king_from).unwrap();
                board.set_piece(king_to, king);
                taken_piece = None;
            }
        }
        if !dry_run {
            self.history.push((chess_move, taken_piece));
            self.legal_moves.clear();
        }
        taken_piece
    }

    pub fn get_legal_moves(&self) -> &Vec<ChessMove> {
        &self.legal_moves
    }

    pub(crate) fn evaluate_legal_moves(&mut self, board: &Board, player: Color) {
        let mut legal_moves = Vec::new();
        for pos in Position::all_iter() {
            let mut legal_moves_from_pos = self.evaluate_legal_moves_from(board, pos, player);
            legal_moves.append(&mut legal_moves_from_pos);
        }

        let mut actual_legal_moves = Vec::new();
        for &m in &legal_moves {
            let mut board = board.clone();
            self.make_move(&mut board, m, true);
            if !self.is_in_check(&board, player) {
                actual_legal_moves.push(m);
            }
        }

        self.legal_moves = actual_legal_moves;
    }

    fn previous_move(&self) -> Option<ChessMove> {
        self.history.last().map(|(m, p)| m).copied()
    }

    fn is_in_check(&self, board: &Board, player: Color) -> bool {
        match player {
            Color::Black => {
                if let Some((pos, piece)) =
                    self.is_under_attack(board, self.black_king, Color::Black)
                {
                    println!("{} is in check by {:?} on {:?}", player, piece, pos);
                    return true;
                }
                false
            }
            Color::White => {
                if let Some((pos, piece)) =
                    self.is_under_attack(board, self.white_king, Color::White)
                {
                    println!("{} is in check by {:?} on {:?}", player, piece, pos);
                    return true;
                }
                false
            }
        }
    }

    fn is_under_attack(
        &self,
        board: &Board,
        target: Position,
        color: Color,
    ) -> Option<(Position, Piece)> {
        use Color::*;
        use PieceType::*;

        // cardinal directions
        {
            let is_cardinal_checker = |pos: Option<Position>| {
                if let Some(pos) = pos {
                    if let Some(piece) = board[pos] {
                        if piece.is_color(color.opponent()) && matches!(piece.kind(), Queen | Rook)
                        {
                            return Some((pos, piece));
                        }
                    }
                }
                return None;
            };

            let up_file = self
                .step_until_collision(target, 1, 0, board, color)
                .last()
                .copied();
            if let Some(checker) = is_cardinal_checker(up_file) {
                return Some(checker);
            }

            let down_file = self
                .step_until_collision(target, -1, 0, board, color)
                .last()
                .copied();
            if let Some(checker) = is_cardinal_checker(down_file) {
                return Some(checker);
            }

            let up_rank = self
                .step_until_collision(target, 0, 1, board, color)
                .last()
                .copied();
            if let Some(checker) = is_cardinal_checker(up_rank) {
                return Some(checker);
            }

            let down_rank = self
                .step_until_collision(target, 0, -1, board, color)
                .last()
                .copied();
            if let Some(checker) = is_cardinal_checker(down_rank) {
                return Some(checker);
            }
        }

        // diagonal directions
        {
            let is_diagonal_checker = |pos: Option<Position>| {
                if let Some(pos) = pos {
                    if let Some(piece) = board[pos] {
                        if piece.is_color(color.opponent())
                            && matches!(piece.kind(), Queen | Bishop)
                        {
                            return Some((pos, piece));
                        }
                    }
                }
                return None;
            };

            let up_file_up_rank = self
                .step_until_collision(target, 1, 1, board, color)
                .last()
                .copied();
            if let Some(checker) = is_diagonal_checker(up_file_up_rank) {
                return Some(checker);
            }

            let up_file_down_rank = self
                .step_until_collision(target, 1, -1, board, color)
                .last()
                .copied();
            if let Some(checker) = is_diagonal_checker(up_file_down_rank) {
                return Some(checker);
            }

            let down_file_up_rank = self
                .step_until_collision(target, -1, 1, board, color)
                .last()
                .copied();
            if let Some(checker) = is_diagonal_checker(down_file_up_rank) {
                return Some(checker);
            }

            let down_file_down_rank = self
                .step_until_collision(target, -1, -1, board, color)
                .last()
                .copied();
            if let Some(checker) = is_diagonal_checker(down_file_down_rank) {
                return Some(checker);
            }
        }

        // knight moves
        {
            let is_knight_checker =
                |piece: Piece| piece.is_color(color.opponent()) && matches!(piece.kind(), Knight);
            for pos in KNIGHT_OFFSETS
                .iter()
                .filter_map(|&(file_step, rank_step)| target.add_offset(file_step, rank_step))
            {
                if let Some(piece) = board[pos] {
                    if is_knight_checker(piece) {
                        return Some((pos, piece));
                    }
                }
            }
        }

        // king moves
        {
            let is_king_checker =
                |piece: Piece| piece.is_color(color.opponent()) && matches!(piece.kind(), King);

            for pos in KING_OFFSETS
                .iter()
                .filter_map(|&(file_step, rank_step)| target.add_offset(file_step, rank_step))
            {
                if let Some(piece) = board[pos] {
                    if is_king_checker(piece) {
                        return Some((pos, piece));
                    }
                }
            }
        }

        // pawn moves
        {
            let is_pawn_checker =
                |piece: Piece| piece.is_color(color.opponent()) && matches!(piece.kind(), Pawn);

            let offsets = match color {
                Black => [(-1, -1), (1, -1)],
                White => [(-1, 1), (1, 1)],
            };

            for pos in offsets
                .iter()
                .filter_map(|&(file_step, rank_step)| target.add_offset(file_step, rank_step))
            {
                if let Some(piece) = board[pos] {
                    if is_pawn_checker(piece) {
                        return Some((pos, piece));
                    }
                }
            }
        }

        None
    }

    fn evaluate_legal_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if let Some(piece) = board[from] {
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
        Vec::new()
    }

    fn evaluate_legal_pawn_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::First || from.rank() == Rank::Eighth {
            // TODO: mark this as an error somehow?
            // There should never be a pawn on the first or eighthranks.
            return Vec::new();
        }
        match player {
            Color::Black => self.evaluate_legal_black_pawn_moves_from(board, from),
            Color::White => self.evaluate_legal_white_pawn_moves_from(board, from),
        }
    }

    fn evaluate_legal_white_pawn_moves_from(
        &self,
        board: &Board,
        from: Position,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::Seventh {
            // from here it's only possible to promote

            let mut moves = Vec::new();

            // check position in front
            let to = Position::new(from.file(), Rank::Eighth);
            if !board.has_piece_at(to) {
                moves.append(&mut ChessMove::promotion_moves(from, to));
            }

            for to in [(-1, 1), (1, 1)]
                .iter()
                .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
            {
                if board.has_piece_with_color_at(to, Color::Black) {
                    moves.append(&mut ChessMove::promotion_moves(from, to));
                }
            }

            return moves;
        } else {
            let mut positions = Vec::new();

            // regular moves, up rank since the pawn is white
            let one_step_forward = from.add_offset(0, 1).unwrap();
            if !board.has_piece_at(one_step_forward) {
                positions.push(one_step_forward);
                if from.rank() == Rank::Second {
                    let two_steps_forward = Position::new(from.file(), Rank::Fourth);
                    if !board.has_piece_at(two_steps_forward) {
                        positions.push(two_steps_forward);
                    }
                }
            }

            if let Some(front_left) = from.add_offset(-1, 1) {
                if board.has_piece_with_color_at(front_left, Color::Black) {
                    positions.push(front_left);
                }
            }

            if let Some(front_right) = from.add_offset(1, 1) {
                if board.has_piece_with_color_at(front_right, Color::Black) {
                    positions.push(front_right);
                }
            }

            let mut legal_moves: Vec<ChessMove> = positions
                .into_iter()
                .map(|to| ChessMove::Regular { from, to })
                .collect();

            if from.rank() == Rank::Fifth {
                if let Some(ChessMove::Regular { from: f, to: t }) = self.previous_move() {
                    let left_file = from.file().add_offset(-1);
                    let right_file = from.file().add_offset(1);
                    for file in [left_file, right_file].iter().filter_map(|&f| f) {
                        if f == Position::new(file, Rank::Seventh)
                            && t == Position::new(file, Rank::Fifth)
                        {
                            legal_moves.push(ChessMove::EnPassant {
                                from,
                                to: Position::new(file, Rank::Sixth),
                                taken_index: Position::new(file, Rank::Fifth),
                            });
                        }
                    }
                }
            }

            legal_moves
        }
    }

    fn evaluate_legal_black_pawn_moves_from(
        &self,
        board: &Board,
        from: Position,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::Second {
            // from here it's only possible to promote

            let mut moves = Vec::new();

            // check position in front
            let to = Position::new(from.file(), Rank::First);
            if !board.has_piece_at(to) {
                moves.append(&mut ChessMove::promotion_moves(from, to));
            }

            for to in [(-1, -1), (1, -1)]
                .iter()
                .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
            {
                if board.has_piece_with_color_at(to, Color::White) {
                    moves.append(&mut ChessMove::promotion_moves(from, to));
                }
            }

            return moves;
        } else {
            let mut positions = Vec::new();

            // regular moves, up rank since the pawn is white
            let one_step_forward = from.add_offset(0, -1).unwrap();
            if !board.has_piece_at(one_step_forward) {
                positions.push(one_step_forward);
                if from.rank() == Rank::Seventh {
                    let two_steps_forward = Position::new(from.file(), Rank::Fifth);
                    if !board.has_piece_at(two_steps_forward) {
                        positions.push(two_steps_forward);
                    }
                }
            }

            if let Some(front_left) = from.add_offset(1, -1) {
                if board.has_piece_with_color_at(front_left, Color::White) {
                    positions.push(front_left);
                }
            }

            if let Some(front_right) = from.add_offset(-1, -1) {
                if board.has_piece_with_color_at(front_right, Color::White) {
                    positions.push(front_right);
                }
            }

            let mut legal_moves: Vec<ChessMove> = positions
                .into_iter()
                .map(|to| ChessMove::Regular { from, to })
                .collect();

            if from.rank() == Rank::Fourth {
                if let Some(ChessMove::Regular { from: f, to: t }) = self.previous_move() {
                    let left_file = from.file().add_offset(-1);
                    let right_file = from.file().add_offset(1);
                    for file in [left_file, right_file].iter().filter_map(|&f| f) {
                        if f == Position::new(file, Rank::Second)
                            && t == Position::new(file, Rank::Fourth)
                        {
                            legal_moves.push(ChessMove::EnPassant {
                                from,
                                to: Position::new(file, Rank::Third),
                                taken_index: Position::new(file, Rank::Fourth),
                            });
                        }
                    }
                }
            }

            legal_moves
        }
    }

    fn evaluate_legal_king_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();
        for to in KING_OFFSETS
            .iter()
            .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        {
            match board.get_piece(to) {
                Option::Some(piece) => {
                    if !piece.is_color(player) {
                        positions.push(to);
                    }
                }
                Option::None => {
                    positions.push(to);
                }
            }
        }

        let mut legal_moves: Vec<ChessMove> = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        match (player, from) {
            (Color::Black, E8) => legal_moves.append(&mut self.evaluate_black_castle(board)),
            (Color::White, E1) => legal_moves.append(&mut self.evaluate_white_castle(board)),
            _ => {}
        }

        legal_moves
    }

    fn evaluate_white_castle(&self, board: &Board) -> Vec<ChessMove> {
        println!("evaluating white castle");
        // has king moved?
        if dbg!(self.history_contains_from_position(E1)) {
            println!("white king has moved");
            return Vec::new();
        }

        let mut moves = Vec::new();

        // check short castle
        if dbg!(!self.history_contains_from_position(H1))
            && dbg!(!board.has_piece_at(F1))
            && dbg!(!board.has_piece_at(G1))
            && dbg!(!self.is_under_attack(board, F1, Color::White).is_some())
            && dbg!(!self.is_under_attack(board, G1, Color::White).is_some())
        {
            moves.push(ChessMove::Castle {
                rook_from: H1,
                rook_to: F1,
                king_from: E1,
                king_to: G1,
            });
        }

        // check long castle
        if dbg!(!self.history_contains_from_position(A1))
            && dbg!(!board.has_piece_at(D1))
            && dbg!(!board.has_piece_at(C1))
            && dbg!(!board.has_piece_at(B1))
            && dbg!(!self.is_under_attack(board, D1, Color::White).is_some())
            && dbg!(!self.is_under_attack(board, C1, Color::White).is_some())
            && dbg!(!self.is_under_attack(board, B1, Color::White).is_some())
        {
            moves.push(ChessMove::Castle {
                rook_from: A1,
                rook_to: D1,
                king_from: E1,
                king_to: C1,
            })
        }

        return moves;
    }

    fn evaluate_black_castle(&self, board: &Board) -> Vec<ChessMove> {
        println!("evaluating black castle");
        // has king moved?
        if self.history_contains_from_position(E8) {
            return Vec::new();
        }

        let mut moves = Vec::new();

        // check short castle
        if !self.history_contains_from_position(H8)
            && !board.has_piece_at(F8)
            && !board.has_piece_at(G8)
            && !self.is_under_attack(board, F8, Color::Black).is_some()
            && !self.is_under_attack(board, G8, Color::Black).is_some()
        {
            moves.push(ChessMove::Castle {
                rook_from: H8,
                rook_to: F8,
                king_from: E8,
                king_to: G8,
            });
        }

        // check long castle
        if !self.history_contains_from_position(A8)
            && !board.has_piece_at(D8)
            && !board.has_piece_at(C8)
            && !board.has_piece_at(B8)
            && !self.is_under_attack(board, D8, Color::Black).is_some()
            && !self.is_under_attack(board, C8, Color::Black).is_some()
            && !self.is_under_attack(board, B8, Color::Black).is_some()
        {
            moves.push(ChessMove::Castle {
                rook_from: A8,
                rook_to: D8,
                king_from: E8,
                king_to: C8,
            })
        }

        return moves;
    }

    fn evaluate_legal_knight_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();
        for to in KNIGHT_OFFSETS
            .iter()
            .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        {
            match board.get_piece(to) {
                Option::Some(piece) => {
                    if !piece.is_color(player) {
                        positions.push(to);
                    }
                }
                Option::None => {
                    positions.push(to);
                }
            }
        }

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_bishop_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();

        let mut up_file_up_rank = self.step_until_collision(from, 1, 1, board, player);
        positions.append(&mut up_file_up_rank);

        let mut up_file_down_rank = self.step_until_collision(from, 1, -1, board, player);
        positions.append(&mut up_file_down_rank);

        let mut down_file_up_rank = self.step_until_collision(from, -1, 1, board, player);
        positions.append(&mut down_file_up_rank);

        let mut down_file_down_rank = self.step_until_collision(from, -1, -1, board, player);
        positions.append(&mut down_file_down_rank);

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_rook_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();

        let mut up_file = self.step_until_collision(from, 1, 0, board, player);
        positions.append(&mut up_file);

        let mut down_file = self.step_until_collision(from, -1, 0, board, player);
        positions.append(&mut down_file);

        let mut up_rank = self.step_until_collision(from, 0, 1, board, player);
        positions.append(&mut up_rank);

        let mut down_rank = self.step_until_collision(from, 0, -1, board, player);
        positions.append(&mut down_rank);

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_queen_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();

        let mut up_file_up_rank = self.step_until_collision(from, 1, 1, board, player);
        positions.append(&mut up_file_up_rank);

        let mut up_file_down_rank = self.step_until_collision(from, 1, -1, board, player);
        positions.append(&mut up_file_down_rank);

        let mut down_file_up_rank = self.step_until_collision(from, -1, 1, board, player);
        positions.append(&mut down_file_up_rank);

        let mut down_file_down_rank = self.step_until_collision(from, -1, -1, board, player);
        positions.append(&mut down_file_down_rank);

        let mut up_file = self.step_until_collision(from, 1, 0, board, player);
        positions.append(&mut up_file);

        let mut down_file = self.step_until_collision(from, -1, 0, board, player);
        positions.append(&mut down_file);

        let mut up_rank = self.step_until_collision(from, 0, 1, board, player);
        positions.append(&mut up_rank);

        let mut down_rank = self.step_until_collision(from, 0, -1, board, player);
        positions.append(&mut down_rank);

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn step_until_collision(
        &self,
        start: Position,
        file_step: i32,
        rank_step: i32,
        board: &Board,
        player: Color,
    ) -> Vec<Position> {
        let mut positions = Vec::new();
        for steps in 1.. {
            if let Some(position) = start.add_offset(file_step * steps, rank_step * steps) {
                match board.get_piece(position) {
                    Option::Some(piece) => {
                        if !piece.is_color(player) {
                            positions.push(position);
                        }
                        break;
                    }
                    Option::None => {
                        positions.push(position);
                    }
                }
            } else {
                break;
            }
        }

        positions
    }

    /// Check if the move history contains any move that was made *from* the given index.
    fn history_contains_from_position(&self, from: Position) -> bool {
        self.history.iter().any(|(m, _)| m.from() == from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;
    use Color::*;

    #[test]
    fn legal_moves() {
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let legal_moves: Vec<_> = manager
            .get_legal_moves()
            .iter()
            .map(|m| (m.from(), m.to()))
            .collect();

        assert_eq!(
            legal_moves,
            vec![
                (A2, A3),
                (A2, A4),
                (B1, C3),
                (B1, A3),
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
                (G1, H3),
                (G1, F3),
                (G2, G3),
                (G2, G4),
                (H2, H3),
                (H2, H4)
            ]
        );
    }

    #[test]
    fn bishop_moves() {
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let bishop_moves_from_f4: Vec<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, F4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_f4, vec![G5, H6, G3, E5, D6, C7, E3]);

        let bishop_moves_from_c1: Vec<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, C1, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_c1, vec![]);
    }

    #[test]
    fn rook_moves() {
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let rook_moves_from_c5: Vec<Position> = manager
            .evaluate_legal_rook_moves_from(&board, C5, Black)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            rook_moves_from_c5,
            vec![D5, E5, F5, G5, H5, B5, A5, C6, C4, C3, C2]
        );
    }

    #[test]
    fn knight_moves() {
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let knight_moves_from_g4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, G4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_g4, vec![H6, F6, E5, E3]);

        let knight_moves_from_d4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, D4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_d4, vec![F5, F3, E6, C6, B5, B3]);
    }

    #[test]
    fn queen_moves() {
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let king_moves_from_a4: Vec<Position> = manager
            .evaluate_legal_king_moves_from(&board, A4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(king_moves_from_a4, vec![A5, B5, B4, B3, A3]);
    }
}
