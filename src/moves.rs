use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Flags {
    pub eat: bool,
    pub king_move: bool,
    pub pawn_move: bool,
    pub rook_move: bool,
    pub bishop_move: bool,
    pub knight_move: bool,
    pub castle: bool,
}

impl Flags {
    pub fn with_eat(&self) -> Self {
        Self { eat: true, ..*self }
    }
}

#[derive(Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    /// Eat, Rook, King, Pawn, Castle
    pub flags: Flags,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.from)
            .field(&self.to)
            .finish()
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        if self.from == other.from && self.to == other.to {
            true
        } else {
            false
        }
    }
}

impl Move {
    pub fn new(from: usize, to: usize, flags: Flags) -> Move {
        Move { from, to, flags }
    }
}
