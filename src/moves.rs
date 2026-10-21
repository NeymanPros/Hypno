use crate::board::Board;
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
    pub fn with_flags(board: &Board, from: usize, to: usize) -> Option<Self> {
        let square = board.get_square(from);
        if from == to {
            return None;
        }
        if ((square & 0b0001_0000) != 0) != board.turn {
            return None;
        }
        let flags = match square & 0b1110_0000 {
            0b1100_0000 => {
                let mut flags = Self::move_rook_possible(&board, from, to);
                if flags.is_some() {
                    flags.as_mut().unwrap().rook_move = true;
                    flags
                } else {
                    None
                }
            }
            0b0110_0000 => Self::move_bishop_possible(board, from, to),
            0b0010_0000 => Self::move_pawn_possible(board, from, to),
            0b0100_0000 => Self::move_knight_possible(board, from, to),
            0b1110_0000 => {
                let flags = Self::move_rook_possible(board, from, to);
                if flags.is_some() {
                    flags
                } else {
                    Self::move_bishop_possible(board, from, to)
                }
            }
            0b1010_0000 => Self::move_king_possible(board, from, to),
            _ => None,
        };

        if flags.is_none() {
            return None;
        }

        Some(Self {
            from,
            to,
            flags: flags.unwrap(),
        })
    }

    fn move_rook_possible(board: &Board, from: usize, to: usize) -> Option<Flags> {
        let mut flags = Flags::default();
        if from / 8 == to / 8 {
            for to_check in (from.min(to) + 1)..from.max(to) {
                if board.is_piece(to_check) {
                    return None;
                }
            }
        } else if from % 8 == to % 8 {
            let mut to_check = from.min(to) + 8;
            while to_check < from.max(to) {
                if board.is_piece(to_check) {
                    return None;
                }
                to_check += 8
            }
        } else {
            return None;
        }

        if board.is_piece(to) {
            if board.is_diff_colors(to, from) {
                flags.eat = true;
            } else {
                return None;
            }
        }

        Some(flags)
    }

    fn move_bishop_possible(board: &Board, from: usize, to: usize) -> Option<Flags> {
        let mut flags = Flags::default();
        let mv = from as i8 - to as i8;

        if mv % 9 == 0 {
            let mut to_check = from.min(to) + 9;
            if to_check / 8 - (to_check - 9) / 8 == 2 {
                return None;
            }
            while to_check < from.max(to) {
                if board.is_piece(to_check) {
                    return None;
                } else if to_check / 8 - (to_check - 9) / 8 == 2 {
                    return None;
                }
                to_check += 9
            }
        } else if mv % 7 == 0 {
            let mut to_check = from.min(to) + 7;
            if to_check / 8 == (to_check - 7) / 8 {
                return None;
            }
            while to_check < from.max(to) {
                if board.is_piece(to_check) {
                    return None;
                } else if to_check / 8 == (to_check - 7) / 8 {
                    return None;
                }
                to_check += 7
            }
        } else {
            return None;
        }

        if board.is_piece(to) {
            if board.is_diff_colors(to, from) {
                flags.eat = true;
            } else {
                return None;
            }
        }

        Some(flags)
    }

    fn move_pawn_possible(board: &Board, from: usize, to: usize) -> Option<Flags> {
        let mut flags = Flags::default();
        flags.pawn_move = true;
        if board.is_white_piece(from) {
            let change = to - from;
            match change {
                8 => {
                    if board.is_piece(to) {
                        return None;
                    }
                }
                16 => {
                    if from / 8 != 1 || board.is_piece(to) || board.is_piece(to - 8) {
                        return None;
                    }
                }
                7 | 9 => {
                    if to / 8 - from / 8 != 1 {
                        // 1 line move
                        return None;
                    } else if board.is_white_piece(to) {
                        // target has same color
                        return None;
                    } else if board.is_black_piece(to) {
                        // target is eatable
                        flags.eat = true;
                    } else if change == 7 && board.get_square(from) == 0b0011_0100
                        || change == 9 && board.get_square(from) == 0b0011_1000
                    {
                        // passant
                        flags.eat = true
                    } else {
                        return None;
                    }
                }

                _ => return None,
            }
        } else {
            // black pawn move
            let change = from - to;
            match change {
                8 => {
                    if board.is_piece(to) {
                        return None;
                    }
                }
                16 => {
                    if from / 8 != 6 || board.is_piece(to) || board.is_piece(to + 8) {
                        return None;
                    }
                }
                7 | 9 => {
                    if from / 8 - to / 8 != 1 {
                        // 1 line move
                        return None;
                    } else if board.is_black_piece(to) {
                        // target has same color
                        return None;
                    } else if board.is_white_piece(to) {
                        // target is eatable
                        flags.eat = true;
                    } else if change == 7 && board.get_square(from) == 0b0010_0100
                        || change == 9 && board.get_square(from) == 0b0010_1000
                    {
                        // passant
                        flags.eat = true;
                    } else {
                        return None;
                    }
                }

                _ => return None,
            }
        }

        flags.pawn_move = true;
        Some(flags)
    }

    fn move_knight_possible(board: &Board, from: usize, to: usize) -> Option<Flags> {
        let mut flags = Flags::default();
        let shift = (
            (from as i8 / 8 - to as i8 / 8).abs(),
            (from as i8 % 8 - to as i8 % 8).abs(),
        );
        match shift {
            (1, 2) | (2, 1) => {
                if board.is_piece(to) {
                    if board.is_diff_colors(to, from) {
                        flags.eat = true;
                    } else {
                        return None;
                    }
                }
            }
            _ => return None,
        }

        Some(flags)
    }

    fn move_king_possible(board: &Board, from: usize, to: usize) -> Option<Flags> {
        let mut flags = Flags::default();
        let shift = (
            (from as i8 / 8 - to as i8 / 8).abs(),
            (from as i8 % 8 - to as i8 % 8).abs(),
        );
        if shift.0.abs() <= 1 && shift.1.abs() <= 1 {
        } else if shift == (0, 2) && board.get_square(from) & 0b0000_1000 != 0 {
            match to {
                1 => {}

                _ => return None,
            }
        } else {
            return None;
        }
        flags.king_move = true;
        Some(flags)
    }

    fn under_checks(board: &Board, king: u8) -> bool {
        false
    }
}
