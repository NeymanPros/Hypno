use crate::Board;
use crate::moves::{Move, Flags};

impl Board {
    /// Adds bishops, rooks and queens which can go onto sq. Can return 1 attack, all attacks or all attacks + xrays.
    pub fn brq_occupy_xrays(&mut self, sq: usize, single_attack: bool, no_xrays: bool, target_color: bool) -> Vec<Move> {
        let mut attacks = vec![];
        
        for add in [-8i8, -1, 1, 8, -9, -7, 7, 9] {
            let mut to_check = sq as i8 + add;
            let  mut king_check = false;
            let mut exit = false;
            let mut maybe_xray = 64;
            
            let cond = match add {
                -1 | 1 => |int: i8, target: i8| {target >> 3 == int >> 3},
                -8 | 8 => |int: i8, _target: i8| {int >= 0 && int <= 63},
                _ => |int: i8, target: i8| {
                    let shift = (((target >> 3) - (int >> 3)).abs(), ((target % 8) - (int % 8)).abs());
                    int >= 0 && int <= 63 && shift.0 == shift.1
                }
            };
            
            let (piece, flags) = match add {
                -8 | -1 | 1 | 8 => (0b1100_0000, Flags{rook_move: true, ..Flags::default()}),
                _ => (0b0110_0000, Flags{bishop_move: true, ..Flags::default()})
            };
            
            while cond(to_check, sq as i8) && !exit {
                let to_check_u = to_check as usize;
                if (self.get_square(to_check_u) & piece) == piece && self.is_white_piece(to_check_u) == target_color {
                    // Target piece with correct color.
                    let move_to_add = Move::new(to_check_u, sq, flags.clone());
                    if king_check {
                        self.xrays.push(Move {
                            to: maybe_xray,
                            ..move_to_add
                        });
                    } else {
                        attacks.push(move_to_add);
                        if single_attack {
                            return attacks;
                        }
                    }
                    exit = true;
                } else if self.is_piece(to_check_u) && self.is_white_piece(to_check_u) == target_color {
                    // Wrong piece with target color.
                    if !king_check {
                        king_check = true;
                        exit = no_xrays;
                        maybe_xray = to_check_u;
                    } else {
                        exit = true;
                    }
                }
                to_check += add;
            }
        }
        attacks
    }
    
    #[inline(always)]
    pub(super) fn knight_occupy(&mut self, sq: usize, knight_white: bool) -> Vec<Move> {
        let mut moves = vec![];
        let knights = self.return_knights(sq);
        for to_check in knights.into_iter() {
            if (self.get_square(to_check) & 0b1110_0000) == 0b0100_0000 && self.is_white_piece(to_check) == knight_white{
                moves.push(Move::new(
                    to_check,
                    sq, 
                    Flags {
                        knight_move: true,
                        ..Flags::default()
                    }
                ))
            }
        }
        moves
    }
    
    pub(super) fn pawn_eat(&mut self, sq: usize, white: bool) -> Vec<Move> {
        if white && sq / 8 <= 1 || !white && sq / 8 >= 6 {
            // Cases where sq is physically unreachable.
            return vec![]
        }
        let pawns = self.find_eat_pawn(sq, white);
        let mut moves = Vec::with_capacity(2);
        for to_check in pawns.into_iter() {
            let move_add = Move::new(to_check, sq, Flags {
                pawn_move: true, 
                eat: true,
                ..Flags::default()
            });
            moves.push(move_add);
        }
        
        moves
    }
    
    /// Pawn exists.
    fn find_eat_pawn(&self, sq: usize, white: bool) -> Vec<usize> {
        let mut pawns = Vec::with_capacity(2);
        if white {
            let pawn = 0b0011_0000;
            // Previous line.
            for sub in [7, 9] {
                let to_check = sq - sub;
                let square = self.get_square(to_check);
                // Pawn in correct row.
                if (square & 0b1111_0000) == pawn && (to_check / 8) + 1 == sq / 8 {
                    pawns.push(to_check)
                }
            }
        } else {
            let pawn = 0b0010_0000;
            for add in [7, 9] {
                let to_check = sq + add;
                let square = self.get_square(to_check);
                // Pawn in correct row.
                if (square & 0b1111_0000) == pawn && to_check / 8 == (sq / 8) + 1 {
                    pawns.push(to_check)
                }
            }
        }
        
        pawns
    }
    
    /// Passant line enter is not possible, as the previous position would be incorrect.
    pub(super) fn pawn_go(&self, sq: usize, white: bool) -> Vec<Move> {
        if white && sq / 8 <= 1 || !white && sq / 8 >= 6 {
            // Cases where sq is physically unreachable.
            return vec![]
        }
        let dir = if white { -8 } else { 8 };
        let pawn = if white {0b0011_0000} else {0b0010_0000};
        let prev_sq = (sq as i8 + dir) as usize;
        if self.get_square(prev_sq) == pawn {
            return vec![Move::new(prev_sq, sq, Flags {
                pawn_move: true,
                ..Flags::default()
            })];
        } else if !self.is_piece(prev_sq) {
            if white && sq / 8 == 3 || !white && sq / 8 == 4 {
                let to_check = (prev_sq as i8 + dir) as usize;
                if self.get_square(to_check) == pawn {
                    // Cannot be more than 1 move.
                    return vec![Move::new(to_check, sq, Flags {
                        pawn_move: true,
                        ..Flags::default()
                    })];
                }
            }
        }
        vec![]
    }
}
