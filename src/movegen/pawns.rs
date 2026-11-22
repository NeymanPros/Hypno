use crate::{Board, Flags, Move};

impl Board {
    pub(super) fn generate_pawns(&mut self, sq: usize) {
        let dir = if self.turn { 8 } else { -8 };

        // If pinned.
        for (num, pin) in self.xrays.iter().enumerate() {
            if sq == pin.to {
                return self.gen_pawns_pinned(sq, dir, self.xrays[num].clone());
            }
        }

        self.straight_pawn_moves(sq, dir);
        self.pawn_eat_moves(sq, dir);
        
        if self.passant {
            self.passants(sq, dir);
        }
    }
    
    #[inline(always)]
    fn straight_pawn_moves(&mut self, sq: usize, dir: i8) {
        let to_check = (sq as i8 + dir) as usize;
        if !self.is_piece(to_check) {
                let flags = Flags {
                pawn_move: true,
                ..Flags::default()
            };
            add_pawn_move(&mut self.all_moves, sq, to_check, flags.clone());

            // Is 2-square move possible.
            if (self.turn && sq / 8 == 1) || (!self.turn && sq / 8 == 6) {
                let to_check = (sq as i8 + dir + dir) as usize;
                if !self.is_piece(to_check) {
                    self.all_moves.push(Move::new(sq, to_check, flags.clone()));
                }
            }
        }
    }

    #[inline(always)]
    fn passants(&mut self, sq: usize, dir: i8) {
        if !self.passant && !self.passant_line_king(sq) {
            return
        }
        let pawn = self.get_square(sq);
        if (pawn & 0b00_0000_10) != 0 {
            let to = (sq as i8 + dir + 1) as usize;
            self.all_moves.push(Move::new(sq, to, Flags {
                pawn_move: true, 
                eat: true, 
                passant: true,
                ..Flags::default()
            }))
        } 
        if (pawn & 0b00_0000_01) != 0 {
            let to = (sq as i8 + dir - 1) as usize;
            self.all_moves.push(Move::new(sq, to, Flags {
                pawn_move: true, 
                eat: true, 
                passant: true,
                ..Flags::default()
            }))
        }
    }
    
    #[inline(always)]
    fn passant_line_king(&self, sq: usize) -> bool {
        let king = if self.turn { self.white_king } else { self.black_king };
        if sq >> 3 == king >> 3 {
            let dir = (king as i8 - sq as i8).signum();
            let mut to_check_i = sq as i8 + dir;
            let mut pieces = 0;
            while to_check_i % 8 == sq as i8 % 8 && pieces < 3 {
                let to_check = to_check_i as usize;
                if self.is_piece(to_check) {
                    pieces += 1;
                    if self.is_diff_colors(king, to_check) && (self.get_square(to_check) & 0b00_1100_00) == 0b00_1100_00 {
                        return false
                    }
                }
                
                to_check_i += dir;
            }
        }
        true
    }

    #[inline(always)]
    fn gen_pawns_pinned(&mut self, sq: usize, dir: i8, pin: Move) {
        if pin.flags.rook_move {
            if pin.from % 8 == pin.to % 8 {
                self.straight_pawn_moves(sq, dir);
            }
        } else {
            self.pawn_eat_pinned(sq, &pin, dir);
            self.passant_pinned(sq, pin, dir);
        }
    }

    #[inline(always)]
    fn pawn_eat_moves(&mut self, sq: usize, dir: i8) {
        let mut to_check = (sq as i8 + dir) as usize;
        if to_check >= 1 {
            to_check -= 1; 
            if self.eatable_by_pawn(to_check, sq) {
                add_pawn_move(&mut self.all_moves, sq, to_check, Flags { 
                    eat: true, 
                    pawn_move: true,
                    ..Flags::default()
                    })
            }
        }

        let to_check = (sq as i8 + dir) as usize + 1;
        if to_check <= 63 {
            if self.eatable_by_pawn(to_check, sq) {
                add_pawn_move(&mut self.all_moves, sq, to_check, Flags { 
                    eat: true, 
                    pawn_move: true,
                    ..Flags::default()
                    })
            }
        }
    }

    #[inline(always)]
    fn eatable_by_pawn(&mut self, to_check: usize, sq: usize) -> bool {
        if self.is_diff_colors(sq, to_check) {
            if (to_check % 8).abs_diff(sq % 8) == 1 {
                // Does not create problems with rook pawns.
                return true
            }
        }
        false
    }
    
    #[inline(always)]
    fn pawn_eat_pinned(&mut self, sq: usize, pin: &Move, dir: i8) {
        let shift = pin.from as i8 - pin.to as i8;
        let add = if shift % 9 == 0 {
            if dir > 0 {
                9i8
            } else {
                -9
            }
        } else {
            if dir > 0 {
                7
            } else {
                -7
            }
        };
        
        // If pinned no overflow possible.
        let to_check = (sq as i8 + add) as usize;
        if self.eatable_by_pawn(to_check, sq) {
                add_pawn_move(&mut self.all_moves, sq, to_check, Flags { 
                    eat: true, 
                    pawn_move: true,
                    ..Flags::default()
                    })
            }
    }
    
    #[inline(always)]
    pub (super) fn passant_pinned(&mut self, sq: usize, pin: Move, dir: i8) {
        if self.passant && pin.flags.bishop_move {
            let row = sq >> 3;
            if row == 3 || row == 4 {
                let shift = pin.from.abs_diff(pin.to);
                // Always positive.
                let pin_step = if shift % 9 == 0 {
                    9i8
                } else if shift % 7 == 0 {
                    7i8
                } else {
                    panic!("Wrong bishop shift!")
                };
                
                let pawn = self.get_square(sq);
                let pawn_step = match pawn & 0b00_0000_11 {
                    0b00_0000_10 => dir + 1,
                    0b00_0000_01 => dir - 1,                
                    _ => return
                };
                
                if pawn_step.abs() == pin_step {
                    self.all_moves.push(Move::new(
                        sq, 
                        (sq as i8 + pawn_step) as usize,
                        Flags {
                            eat: true,
                            pawn_move: true,
                            passant: true,
                            ..Flags::default()
                        }
                    ))
                }
            }
        }
    }
}

#[inline(always)]
pub(super) fn add_pawn_move(moves: &mut Vec<Move>, from: usize, to: usize, flags: Flags) {
    if (to >> 3) == 7 || (to >> 3) == 0 {
        moves.push(Move::new(
            from, to,
            Flags {
                bishop_move: true,
                rook_move: true,
                ..flags
            },
        ));
        moves.push(Move::new(
            from, to,
            Flags {
                rook_move: true,
                ..flags
            },
        ));
        moves.push(Move::new(
            from, to,
            Flags {
                bishop_move: true,
                ..flags
            },
        ));
        moves.push(Move::new(
            from, to,
            Flags {
                knight_move: true,
                ..flags
            },
        ));
    } else {
        moves.push(Move::new(from, to, flags))
    }
}
