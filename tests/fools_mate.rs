use chess::*;

fn main() {
    king_is_checked();
}

#[test]
fn king_is_checked() {
    let mut game = Game::new();

    game.make_move(ChessMove::regular(F2, F3)).unwrap();
    game.make_move(ChessMove::regular(E7, E5)).unwrap();
    game.make_move(ChessMove::regular(G2, G4)).unwrap();
    game.make_move(ChessMove::regular(D8, H4)).unwrap();
    assert!(game.is_king_checked(Color::White));
}
