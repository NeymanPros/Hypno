use crate::{Board, Move, Flags};

impl Board {
    /// Adds bishops, rooks and queens which can go onto sq. Can return 1 attack, all attacks or all attacks + xrays.
    pub fn brq_occupy_xrays(&mut self, sq: usize, single_attack: bool, ignore_xrays: bool, target_color: bool) -> Vec<Move> {
        let mut attacks = vec![];
        
        for add in [-8i8, -1, 1, 8, -9, -7, 7, 9] { 
            let mut to_check = sq as i8 + add; 
            let  mut king_check = false;
            let mut exit = false;
            let mut maybe_xray = 64;
            
            let cond = match add {
                -1 | 1 => |start: i8, target: i8| {target >> 3 == start >> 3},
                -8 | 8 => |start: i8, _target: i8| {start >= 0 && start <= 63},
                _ => |start: i8, target: i8| {
                    let shift_0 = (target >> 3).abs_diff(start >> 3);
                    let shift_1 = (target % 8).abs_diff(start % 8); 
                    start >= 0 && start <= 63 && shift_0 == shift_1
                }
            };
            
            let (piece, flags) = match add {
                -8 | -1 | 1 | 8 => (0b00_1100_00, Flags{rook_move: true, ..Flags::default()}),
                _ => (0b00_0110_00, Flags{bishop_move: true, ..Flags::default()}) 
            };
            
            while cond(to_check, sq as i8) && !exit { 
                let to_check_u = to_check as usize;
                if !self.is_piece(to_check_u) {
                    // If not a piece
                    to_check += add;
                    continue;
                }
                if self.is_white_piece(to_check_u) == target_color {
                    // Corrent color
                    exit = true;

                    if (self.get_square(to_check_u) & piece) == piece {
                        // Target piece
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
                    }
                } else {
                    // Wrong color, can be pinned.
                    if !king_check {
                        king_check = true;
                        exit = ignore_xrays;
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
    pub(super) fn knight_occupy(&mut self, sq: usize, is_knight_white: bool) -> Vec<Move> {
        let mut moves = vec![];
        let knights = self.return_knights(sq);
        for to_check in knights.into_iter() {
            if (self.get_square(to_check) & 0b00_1111_00) == 0b00_0100_00 && self.is_white_piece(to_check) == is_knight_white {
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
    
    // Returns Vec of possible moves.
    pub(crate) fn pawn_eat(&mut self, sq: usize, white: bool) -> Vec<Move> {
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
            let pawn = 0b10_0001_00;
            // Previous line.
            for sub in [7, 9] {
                let to_check = sq - sub;
                let square = self.get_square(to_check);
                // Pawn in correct row.
                if (square & 0b10_0001_00) == pawn && (to_check >> 3) + 1 == (sq >> 3) {
                    pawns.push(to_check)
                }
            }
        } else {
            let pawn = 0b01_0001_00;
            for add in [7, 9] {
                let to_check = sq + add;
                let square = self.get_square(to_check);
                // Pawn in correct row.
                if (square & 0b01_1111_00) == pawn && (to_check >> 3) == (sq >> 3) + 1 {
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
        let pawn = if white {0b10_0001_00} else {0b01_0001_00};
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
