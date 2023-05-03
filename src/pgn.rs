use crate::{chess_move::PromotionPiece, prelude::*};
use regex::Regex;
use std::{collections::HashMap, convert::TryFrom, str::FromStr};

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
    type Err = u32;

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

        todo!()
    }
}

fn parse_tags(input: &str) -> Result<PgnTags, i32> {
    let lines: Vec<_> = input.lines().collect();
    let event = tag_regex(r#"^\[Event\s+\"(?P<event>.+)\"\]$"#, "event", &lines[0])?;
    let site = tag_regex(r#"^\[Site\s+\"(?P<site>.+)\"\]$"#, "site", &lines[1])?;
    let date = tag_regex(r#"^\[Date\s+\"(?P<date>.+)\"\]$"#, "date", &lines[2])?;
    let round = tag_regex(r#"^\[Round\s+\"(?P<round>.+)\"\]$"#, "round", &lines[3])?;
    let white = tag_regex(r#"^\[White\s+\"(?P<white>.+)\"\]$"#, "white", &lines[4])?;
    let black = tag_regex(r#"^\[Black\s+\"(?P<black>.+)\"\]$"#, "black", &lines[5])?;
    let result = tag_regex(r#"^\[Result\s+\"(?P<result>.+)\"\]$"#, "result", &lines[6])?;
    let extra_regex = Regex::new(r#"^\[(?P<key>.+)\s+\"(?P<value>.+)\"\]$"#).unwrap();
    let mut extra = HashMap::new();
    for &s in &lines[7..] {
        if s.is_empty() {
            break;
        }
        let cap = extra_regex.captures(s).ok_or(3)?;
        let key = cap.name("key").ok_or(4)?;
        let value = cap.name("value").ok_or(5)?;
        extra.insert(key.as_str().to_owned(), value.as_str().to_owned());
    }

    Ok(PgnTags::new(
        event.to_string(),
        site.to_string(),
        date.to_string(),
        round.to_string(),
        white.to_string(),
        black.to_string(),
        result.to_string(),
        extra,
    ))
}

fn tag_regex<'a>(regex: &'a str, capture: &str, text: &'a str) -> Result<&'a str, i32> {
    Ok(Regex::new(regex)
        .expect("invalid regex")
        .captures(text)
        .ok_or(1)?
        .name(capture)
        .ok_or(2)?
        .as_str())
}

fn parse_moves(s: &str) -> Result<Vec<PgnMove>, i32> {
    let mut moves = Vec::new();
    let mut idx = 0;

    let chars: Vec<_> = s.chars().collect();
    loop {
        let move_number = parse_move_number(s, &chars, &mut idx)?;
        eat_whitespace(s, &chars, &mut idx);
        let comment = parse_comment(s, &chars, &mut idx)?;
        eat_whitespace(s, &chars, &mut idx);
    }

    Ok(moves)
}

fn eat_whitespace(s: &str, chars: &[char], idx: &mut usize) {
    while chars[*idx].is_whitespace() {
        *idx += 1;
    }
}

fn parse_move_number(s: &str, chars: &[char], idx: &mut usize) -> Result<usize, i32> {
    let start = *idx;
    let mut end = *idx;
    while chars[*idx].is_digit(10) {
        end += 1;
    }

    s[start..end].parse::<usize>().map_err(|_| 1)
}

fn parse_comment<'a>(
    s: &'a str,
    chars: &'a [char],
    idx: &'a mut usize,
) -> Result<Option<&'a str>, i32> {
    let start = *idx;
    let mut end = *idx;
    if chars[start] == '{' {
        // block comment
        let end_comment = s[start + 1..].find('}').ok_or(5)?;
        end = end_comment + 1;
        return Ok(Some(&s[start..end_comment]));
    } else if chars[start] == ';' {
        let mut end_comment = start + 1;
        for c in end_comment..chars.len() {
            if chars[c] == '\n' {
                return Ok(Some(&s[start + 1..c]));
            }
        }
    }
    return Err(6);
}

fn parse_move(s: &str, chars: &[char], idx: &mut usize) -> Result<PgnMove, i32> {
    let start = *idx;
    let mut end = start;

    for i in start..chars.len() {
        end = i;
        if chars[i].is_whitespace() {
            break;
        }
    }

    let pgn_move = PgnMove::from_str(&s[start..end])?;

    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PgnTags {
    event: String,
    site: String,
    date: String,
    round: String,
    white: String,
    black: String,
    result: String,
    extra: HashMap<String, String>,
}

impl PgnTags {
    pub(crate) fn new(
        event: String,
        site: String,
        date: String,
        round: String,
        white: String,
        black: String,
        result: String,
        extra: HashMap<String, String>,
    ) -> Self {
        Self {
            event,
            site,
            date,
            round,
            white,
            black,
            result,
            extra,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PgnMove {
    kind: PgnMoveKind,
    takes: bool,
    check: bool,
    check_mate: bool,
}

impl FromStr for PgnMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<_> = s.chars().collect();

        let mut idx = 0;

        // check if a piece is moved
        let piece_type = match c[idx] {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'R' => PieceType::Rook,
            'B' => PieceType::Bishop,
            'N' => PieceType::Knight,
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => PieceType::Pawn,
            e => return Err(format!("invalid piece type '{}'", e)),
        };

        if piece_type != PieceType::Pawn {
            idx += 1;

            // check if a file or rank is specified
            let (file, rank) = match c[idx] {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {
                    (Some(File::try_from(c[idx]).unwrap()), None)
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    (None, Some(Rank::try_from(c[idx]).unwrap()))
                }
            };
        }

        todo!()
    }
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

    #[test]
    fn parse_tags_test() {
        let tags = parse_tags(
            r#"[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]
[ExtraTag "ExtraTagValue"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2"#,
        )
        .unwrap();

        assert_eq!(
            tags,
            PgnTags::new(
                "F/S Return Match".to_string(),
                "Belgrade, Serbia JUG".to_string(),
                "1992.11.04".to_string(),
                "29".to_string(),
                "Fischer, Robert J.".to_string(),
                "Spassky, Boris V.".to_string(),
                "1/2-1/2".to_string(),
                vec![("ExtraTag".to_string(), "ExtraTagValue".to_string())]
                    .into_iter()
                    .collect()
            )
        );
    }
}
