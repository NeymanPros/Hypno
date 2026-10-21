use crate::Board;
use crate::moves::{Flags, Move};

impl Board {
    pub fn generate_pawns(&mut self, sq: usize) {
        let dir = if self.turn { 8 } else { -8 };

        for i in 0..self.xrays.len() {
            if self.piece_in_xray(sq, &self.xrays[i]) {
                return self.gen_pawns_pinned(sq, dir, self.xrays[i].clone());
            }
        }

        let flags = Flags {
            pawn_move: true,
            ..Flags::default()
        };
        let to_check = (sq as i8 + dir) as usize;

        // Is 1-square move is possible.
        if !self.is_piece(to_check) {
            self.add_pawn_move(sq, to_check, flags.clone());

            // Is 2-square move is possible.
            if (self.turn && sq / 8 == 1) || (!self.turn && sq / 8 == 6) {
                let to_check = (sq as i8 + dir + dir) as usize;
                if !self.is_piece(to_check) {
                    self.all_moves.push(Move::new(sq, to_check, flags.clone()));
                }
            }
        }

        self.pawn_eat_moves(sq, dir);
        if self.passant {
            self.passants(sq, dir);
        }
    }

    #[inline(always)]
    fn passants(&mut self, sq: usize, dir: i8) {
        let pawn = self.get_square(sq);
        if (pawn & 0b0000_1000) != 0 {
            
        } 
        if (pawn & 0b0000_0100) != 0 {
            
        }
    }

    #[inline(always)]
    fn gen_pawns_pinned(&mut self, sq: usize, dir: i8, pin: Move) {
        if pin.flags.rook_move {
            if pin.from % 8 == pin.to % 8 {
                let to_check = (sq as i8 + dir) as usize;
                if !self.is_piece(to_check) {
                    self.add_pawn_move(
                        sq,
                        to_check,
                        Flags {
                            pawn_move: true,
                            ..Flags::default()
                        },
                    )
                }
            }
        } else {
            self.pawn_eat_moves(sq, dir);
        }
    }

    #[inline(always)]
    fn pawn_eat_moves(&mut self, sq: usize, dir: i8) {
        let mut to_check = (sq as i8 + dir) as usize;
        if to_check >= 1 {
            to_check -= 1;
            self.eatable_by_pawn(
                to_check,
                sq,
                Flags {
                    pawn_move: true,
                    eat: true,
                    ..Flags::default()
                },
            );
        }

        let to_check = (sq as i8 + dir) as usize + 1;
        if to_check <= 63 {
            self.eatable_by_pawn(
                to_check,
                sq,
                Flags {
                    pawn_move: true,
                    eat: true,
                    ..Flags::default()
                },
            );
        }
    }

    #[inline(always)]
    fn eatable_by_pawn(&mut self, to_check: usize, sq: usize, flags: Flags) {
        if self.is_piece(to_check) && self.is_diff_colors(sq, to_check) {
            if (to_check >> 3) + 1 == sq >> 3 || to_check >> 3 == (sq >> 3) + 1 {
                // Can be -2, -1, 0, 1, 2 - only 1 is to searched.
                self.add_pawn_move(sq, to_check, Flags { eat: true, ..flags })
            }
        }
    }

    #[inline(always)]
    fn add_pawn_move(&mut self, from: usize, to: usize, flags: Flags) {
        if to / 8 == 7 || to / 8 == 0 {
            self.all_moves.push(Move::new(
                from, to,
                Flags {
                    bishop_move: true,
                    rook_move: true,
                    ..flags
                },
            ));
            self.all_moves.push(Move::new(
                from, to,
                Flags {
                    rook_move: true,
                    ..flags
                },
            ));
            self.all_moves.push(Move::new(
                from, to,
                Flags {
                    bishop_move: true,
                    ..flags
                },
            ));
            self.all_moves.push(Move::new(
                from, to,
                Flags {
                    knight_move: true,
                    ..flags
                },
            ));
        } else {
            self.all_moves.push(Move::new(from, to, flags))
        }
    }
}
