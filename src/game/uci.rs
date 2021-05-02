use std::{convert::TryFrom, str::FromStr};

use crate::{chess_move::ChessMove, game::Game, File, Position, Rank};

pub struct Uci {
    game: Game,
}

impl Uci {
    pub fn run(mut self) {
        self.identify();
        self.uciok();

        loop {
            self.handle_input();
        }
    }

    fn handle_input(&mut self) {
        use InputCommand::*;
        match read_line() {
            (IsReady, args) => {
                self.ready_ok();
                return;
            }
            (UciNewGame, args) => {
                self.handle_input();
            }
            (Position, args) => {
                self.handle_position(args);
                self.handle_input();
            }
        }
    }

    fn handle_position(&mut self, args: Vec<String>) {
        if args.get(1) == Some(&String::from("startpos")) {
            self.game = Game::new();
            if args.get(2) == Some(&String::from("moves")) {
                for m in &args[3..] {
                    let chess_move = self.parse_move(m).unwrap();
                    self.game.make_move(chess_move).unwrap();
                }
            }
        }
    }

    fn ready_ok(&mut self) {
        println!("readyok");
    }

    fn identify(&mut self) {
        println!("id name ChessRs");
        println!("id author Isak Jägberg");
    }

    fn uciok(&mut self) {
        println!("uciok");
    }

    fn parse_move(&self, s: &str) -> Result<ChessMove, String> {
        let chars: Vec<_> = s.chars().collect();

        let chunks: Vec<_> = chars.chunks(2).collect();

        let file = File::try_from(chunks[0][0])
            .map_err(|_| format!("failed to parse {} as file", chunks[0][0]))?;
        let rank = Rank::try_from(chunks[0][1])
            .map_err(|_| format!("failed to parse {} as rank", chunks[0][1]))?;
        let from = Position::new(file, rank);

        let file = File::try_from(chunks[1][0])
            .map_err(|_| format!("failed to parse {} as file", chunks[1][0]))?;
        let rank = Rank::try_from(chunks[1][1])
            .map_err(|_| format!("failed to parse {} as rank", chunks[1][1]))?;
        let to = Position::new(file, rank);

        dbg!(from, to);

        let moves = self.game.move_manager.get_legal_moves();
        let mut chosen_move = None;
        for &m in moves {
            if (from, to) == (m.from(), m.to()) {
                chosen_move = Some(m);
                break;
            }
        }

        Ok(chosen_move.ok_or("no move chosen".to_string())?)
    }
}

fn read_line() -> (InputCommand, Vec<String>) {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let input = buffer.trim().to_owned();

    let parts: Vec<String> = input.split(" ").map(|p| p.to_owned()).collect();
    let cmd: InputCommand = parts[0].parse().unwrap();
    (cmd, parts)
}

enum InputCommand {
    IsReady,
    UciNewGame,
    Position,
}

impl FromStr for InputCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InputCommand::*;
        Ok(match s {
            "isready" => IsReady,
            "ucinewgame" => UciNewGame,
            "position" => Position,
            invalid => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn run_example() {
        let mut uci = Uci { game: Game::new() };
        uci.handle_position(vec![
            "position".to_string(),
            "startpos".to_string(),
            "moves".to_string(),
            "e2e4".to_string(),
        ]);

        assert!(uci.game.board.get_piece(E4).is_some());
    }
}
