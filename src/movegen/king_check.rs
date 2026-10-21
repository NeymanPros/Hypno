use crate::Board;
use crate::moves::{Move, Flags};

impl Board {
    /// Direct attacks. Use 1100 for rooks and 0110 for bishops
    pub fn king_check(&mut self) {
        let king = if self.turn { self.white_king  } else { self.black_king };
        self.knight_checks(king);
        let mut attacks = self.add_checks_xrays(king, true);
        self.checks.append(&mut attacks);
    }

    fn knight_checks(&mut self, king: usize) {
        let add_what = |board: &Self, sq: usize, to_check: usize| {
            if (board.get_square(to_check) & 0b1110_0000) == 0b0100_0000 && board.is_diff_colors(sq, to_check) {
                Some(Flags {
                    knight_move: true,
                    ..Flags::default()
                })
            } else {
                None
            }
        };
        let knights = self.return_knights(king, add_what);
        // Swap because king doesn't have to move.
        knights.into_iter().for_each(|n| {
            self.checks.push(Move::new(
                n.to,
                n.from,
                n.flags
            ))
        })
    }
    
    /// Returns Vec of checks.
    fn add_checks_xrays(&mut self, king: usize, single_check: bool) -> Vec<Move> {
        let mut attacks = vec![];
        
        for add in [-8i8, -1, 1, 8, -9, -7, 7, 9] {
            let mut to_check = king as i8 + add;
            let  mut check = false;
            let mut xray = false;
            
            let cond = match add {
                -1 | 1 => |int: i8, king_i: i8| {king_i >> 3 == int >> 3},
                -8 | 8 => |int: i8, _king_i: i8| {int >= 0 && int <= 63},
                _ => |int: i8, king_i: i8| {
                    let shift = (((king_i >> 3) - (int >> 3)).abs(), ((king_i % 8) - (int % 8)).abs());
                    int >= 0 && int <= 63 && shift.0 == shift.1
                }
            };
            
            let (piece, flags) = match add {
                -8 | -1 | 1 | 8 => (0b1100_0000, Flags{rook_move: true, ..Flags::default()}),
                _ => (0b0110_0000, Flags{bishop_move: true, ..Flags::default()})
            };
            
            while cond(to_check, king as i8) && !xray {
                let to_check_u = to_check as usize;
                if (self.get_square(to_check_u) & piece) == piece && self.is_diff_colors(king, to_check_u) {
                    let move_to_add = Move::new(to_check_u, king, flags.clone());
                    if check {
                        self.xrays.push(move_to_add);
                    } else {
                        attacks.push(move_to_add);
                        if single_check {
                            return attacks;
                        }
                    }
                    xray = true;
                } else if self.is_piece(to_check_u) && !self.is_diff_colors(king, to_check_u) {
                    if !check {
                        check = true;
                    } else {
                        xray = true;
                    }
                }
                to_check += add;
            }
        }
        attacks
    }

    pub fn piece_in_xray(&self, sq: usize, mv: &Move) -> bool {
        if mv.flags.rook_move {
            false
        } else if mv.flags.bishop_move {
            false
        } else {
            false
        }
    }
}
