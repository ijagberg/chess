use std::{convert::TryFrom, str::FromStr};

use crate::{chess_move::ChessMove, game::Game, File, Position, Rank};

pub struct Uci {
    handler: CommandHandler,
}

impl Uci {
    pub fn run(mut self) {
        loop {
            let (cmd, args) = read_line();
            match self.handler.handle(cmd, args) {
                CommandResult::Provide(outputs) => {
                    for output in outputs {
                        println!("{}", output);
                    }
                }
                CommandResult::Expecting => {
                    continue;
                }
            }
        }
    }
}

struct CommandHandler {
    game: Game,
}

impl CommandHandler {
    pub fn handle(&mut self, cmd: InputCommand, args: Vec<String>) -> CommandResult {
        use CommandResult::*;
        match cmd {
            InputCommand::IsReady => Provide(vec!["readyok".to_string()]),
            InputCommand::UciNewGame => {
                self.game = Game::new();
                Expecting
            }
            InputCommand::Position => {
                todo!()
            }
            InputCommand::Uci => Provide(vec![
                "id name Chessrs".to_string(),
                "id author Isak Jägberg".to_string(),
                "uciok".to_string(),
            ]),
        }
    }
}

enum CommandResult {
    Provide(Vec<String>),
    Expecting,
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
    Uci,
}

impl FromStr for InputCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InputCommand::*;
        Ok(match s {
            "uci" => Uci,
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
}
