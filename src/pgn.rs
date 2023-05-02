use regex::Regex;

use crate::{chess_move::PromotionPiece, prelude::*};
use std::{collections::HashMap, str::FromStr};

pub(crate) struct Pgn {
    tags: PgnTags,
    moves: Vec<PgnMove>,
}

impl Pgn {
    pub fn get_game(self) -> Result<Game, ()> {
        let mut game = Game::default();
        for c in self.moves.chunks(2) {
            let whites_move = c[0];
            match whites_move.kind {
                PgnMoveKind::Regular {
                    piece_type,
                    file,
                    rank,
                    target,
                } => todo!(),
                PgnMoveKind::KingSideCastle => todo!(),
                PgnMoveKind::QueenSideCastle => todo!(),
                PgnMoveKind::Promotion {
                    target,
                    promotion_piece,
                } => todo!(),
            }
        }

        todo!()
    }
}

impl FromStr for Pgn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // [Event "F/S Return Match"]
        // [Site "Belgrade, Serbia JUG"]
        // [Date "1992.11.04"]
        // [Round "29"]
        // [White "Fischer, Robert J."]
        // [Black "Spassky, Boris V."]
        // [Result "1/2-1/2"]

        // 1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
        // 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
        // 11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
        // Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
        // 23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
        // hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
        // 35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
        // Nf2 42. g4 Bd3 43. Re6 1/2-1/2

        let lines: Vec<_> = s.lines().collect();
        let event = Regex::new(r#"^\[Event\s+\"(?P<event>.+)\"\]$"#)
            .expect("invalid regex")
            .captures(&lines[0])
            .ok_or(())?
            .name("event")
            .ok_or(())?
            .as_str();
        let site = Regex::new(r#"^\[Site\s+\"(?P<site>.+)\"\]$"#)
            .expect("invalid regex")
            .captures(&lines[1])
            .ok_or(())?
            .name("site")
            .ok_or(())?
            .as_str();
        let date = Regex::new(r#"^\[Date\s+\"(?P<date>.+)\"\]$"#)
            .expect("invalid regex")
            .captures(&lines[2])
            .ok_or(())?
            .name("date")
            .ok_or(())?
            .as_str();
        let round = Regex::new(r#"^\[Round\s+\"(?P<round>.+)\"\]$"#)
            .expect("invalid regex")
            .captures(&lines[3])
            .ok_or(())?
            .name("round")
            .ok_or(())?
            .as_str();

        todo!()
    }
}

pub(crate) struct PgnTags {
    event: Option<String>,
    site: Option<String>,
    date: Option<String>,
    round: Option<String>,
    white: Option<String>,
    black: Option<String>,
    result: Option<String>,
    extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PgnMove {
    kind: PgnMoveKind,
    takes: bool,
    check: bool,
    check_mate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PgnMoveKind {
    Regular {
        piece_type: PieceType,
        file: Option<File>,
        rank: Option<Rank>,
        target: Position,
    },
    KingSideCastle,
    QueenSideCastle,
    Promotion {
        target: Position,
        promotion_piece: PromotionPiece,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_test() {
        let cap = Regex::new(r#"^\[Event\s+\"(?P<event>.+)\"\]$"#)
            .unwrap()
            .captures(r#"[Event "F/S Return Match"]"#)
            .unwrap();
        assert_eq!(cap.name("event").unwrap().as_str(), "F/S Return Match");
    }
}
