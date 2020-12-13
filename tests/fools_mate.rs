use chess::*;

fn main() {
    let mut game = Game::new();

    game.execute_move(ChessMove::regular(F2, F3));
    game.execute_move(ChessMove::regular(E7, E5));
    game.execute_move(ChessMove::regular(G2, G4));
    game.execute_move(ChessMove::regular(D8, H4));

    assert!(game.is_king_checked(Color::White));
}
