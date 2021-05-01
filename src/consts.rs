use crate::{File, Position, Rank};

pub const A1: Position = Position(File::A, Rank::First);
pub const A2: Position = Position(File::A, Rank::Second);
pub const A3: Position = Position(File::A, Rank::Third);
pub const A4: Position = Position(File::A, Rank::Fourth);
pub const A5: Position = Position(File::A, Rank::Fifth);
pub const A6: Position = Position(File::A, Rank::Sixth);
pub const A7: Position = Position(File::A, Rank::Seventh);
pub const A8: Position = Position(File::A, Rank::Eighth);

pub const B1: Position = Position(File::B, Rank::First);
pub const B2: Position = Position(File::B, Rank::Second);
pub const B3: Position = Position(File::B, Rank::Third);
pub const B4: Position = Position(File::B, Rank::Fourth);
pub const B5: Position = Position(File::B, Rank::Fifth);
pub const B6: Position = Position(File::B, Rank::Sixth);
pub const B7: Position = Position(File::B, Rank::Seventh);
pub const B8: Position = Position(File::B, Rank::Eighth);

pub const C1: Position = Position(File::C, Rank::First);
pub const C2: Position = Position(File::C, Rank::Second);
pub const C3: Position = Position(File::C, Rank::Third);
pub const C4: Position = Position(File::C, Rank::Fourth);
pub const C5: Position = Position(File::C, Rank::Fifth);
pub const C6: Position = Position(File::C, Rank::Sixth);
pub const C7: Position = Position(File::C, Rank::Seventh);
pub const C8: Position = Position(File::C, Rank::Eighth);

pub const D1: Position = Position(File::D, Rank::First);
pub const D2: Position = Position(File::D, Rank::Second);
pub const D3: Position = Position(File::D, Rank::Third);
pub const D4: Position = Position(File::D, Rank::Fourth);
pub const D5: Position = Position(File::D, Rank::Fifth);
pub const D6: Position = Position(File::D, Rank::Sixth);
pub const D7: Position = Position(File::D, Rank::Seventh);
pub const D8: Position = Position(File::D, Rank::Eighth);
pub const E1: Position = Position(File::E, Rank::First);

pub const E2: Position = Position(File::E, Rank::Second);
pub const E3: Position = Position(File::E, Rank::Third);
pub const E4: Position = Position(File::E, Rank::Fourth);
pub const E5: Position = Position(File::E, Rank::Fifth);
pub const E6: Position = Position(File::E, Rank::Sixth);
pub const E7: Position = Position(File::E, Rank::Seventh);
pub const E8: Position = Position(File::E, Rank::Eighth);

pub const F1: Position = Position(File::F, Rank::First);
pub const F2: Position = Position(File::F, Rank::Second);
pub const F3: Position = Position(File::F, Rank::Third);
pub const F4: Position = Position(File::F, Rank::Fourth);
pub const F5: Position = Position(File::F, Rank::Fifth);
pub const F6: Position = Position(File::F, Rank::Sixth);
pub const F7: Position = Position(File::F, Rank::Seventh);
pub const F8: Position = Position(File::F, Rank::Eighth);

pub const G1: Position = Position(File::G, Rank::First);
pub const G2: Position = Position(File::G, Rank::Second);
pub const G3: Position = Position(File::G, Rank::Third);
pub const G4: Position = Position(File::G, Rank::Fourth);
pub const G5: Position = Position(File::G, Rank::Fifth);
pub const G6: Position = Position(File::G, Rank::Sixth);
pub const G7: Position = Position(File::G, Rank::Seventh);
pub const G8: Position = Position(File::G, Rank::Eighth);

pub const H1: Position = Position(File::H, Rank::First);
pub const H2: Position = Position(File::H, Rank::Second);
pub const H3: Position = Position(File::H, Rank::Third);
pub const H4: Position = Position(File::H, Rank::Fourth);
pub const H5: Position = Position(File::H, Rank::Fifth);
pub const H6: Position = Position(File::H, Rank::Sixth);
pub const H7: Position = Position(File::H, Rank::Seventh);
pub const H8: Position = Position(File::H, Rank::Eighth);

pub const INCREASING_ORDER: [Position; 64] = [
    A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4, E5, E6, E7, E8, F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8, H1, H2, H3, H4, H5, H6, H7, H8,
];
