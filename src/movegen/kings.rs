use crate::{Board, Move, Flags};

impl Board {
    #[inline(always)]
    pub(super) fn get_king(&self, white: bool) -> usize {
        if white { self.white_king } else { self.black_king }
    }
    
    pub(super) fn generate_king(&mut self, sq: usize) {
        let mut ensure = self.gen_all_kings(sq as i8);

        // Not a piece or diff colors.
        ensure.retain(|&add| {
            !self.is_piece(add) || self.is_diff_colors(sq, add)
        });
        
        // If in check.
        self.checks.iter().for_each(|check| remove_checked(&mut ensure, sq, check));

        // No steps onto attacked sq.
        ensure.retain(|&to_check| {
            let attacks = self.go_square(to_check, true, !self.turn, true, true);
            attacks.len() == 0
        });
        
        // Enemy king collisions.
        let enemy_sq = self.get_king(!self.turn) as i8;
        let enemy_moves = self.gen_all_kings(enemy_sq);
        ensure.retain(|to_check| !enemy_moves.contains(to_check) );

        // Get castle check.
        let king = self.get_square(sq);
        let castle_possible = if (king & 0b00_0000_10) != 0 && self.checks.len() == 0 {
            true
        } else {
            false
        };

        // Add king moves.
        for to in ensure {
            let flags = Flags {
                king_move: true,
                eat: self.is_piece(to),
                ..Flags::default()
            };
            self.all_moves.push(Move::new(
                sq, 
                to,
                flags
            ));
            
            if castle_possible {
                self.castle_add(sq, to)
            }
        }
    }
    
    // All possible moves.
    #[inline(always)]
    fn gen_all_kings(&self, sq: i8) -> Vec<usize> {
        let pos= (sq >> 3, sq % 8);
        let dirs = match pos {
            (0, 0) => vec![1i8, 8, 9],
            (0, 7) => vec![-1, 7, 8],
            (7, 0) => vec![1, -8, -7],
            (7, 7) => vec![-1, -8, -9],
            (0, _) => vec![-1, 1, 7, 8, 9],
            (7, _) => vec![-1, 1, -7, -8, -9],
            (_, 0) => vec![1, -8, 8, -7, 9],
            (_, 7) => vec![-1, -8, -8, -9, 7],
            _ => vec![-9, -8, -7, -1, 1, 7, 8, 9]
        };
        
        dirs.into_iter().map(|add| {
            (sq + add) as usize
        }).collect()
    }
    
    #[inline(always)]
    fn castle_add(&mut self, sq: usize, to: usize) {
        let diff = to as i8 - sq as i8;
        let (rook_sq, final_sq) = match diff {
            -1 => {
                if self.turn { (0, 2) } else { (56, 58) }
            },
            1 => {
                if self.turn { (7, 6) } else { (63, 62) }
            },
            _ => return
        };
        
        // Rook can castle and square is empty.
        if self.slice_castle(rook_sq.min(sq)) {
            let attacks = self.go_square(final_sq, true, !self.turn, true, true);
            if attacks.len() == 0 {
                self.all_moves.push(Move::new (
                    sq, 
                    final_sq,
                    Flags {
                        king_move: true, 
                        castle: true,
                        ..Flags::default()
                    }
                ))
            }
        }
    }
    
    /// Compares 4 squares.
    fn slice_castle(&self, from: usize) -> bool {
        let slice_32 = u32::from_be_bytes([
            self.get_square(from),
            self.get_square(from + 1),
            self.get_square(from + 2),
            self.get_square(from + 3)
        ]);
        
        // Queen side
        (slice_32 & 0b00_1100_10_11111111_11111111_11111111) == 0b00_1100_10_00000000_00000000_00000000
        ||
        // King side
        (slice_32 & 0b00_1111_10_11111111_11111111_00_1111_10) == 0b00_1000_10_00000000_00000000_00_1100_10
    }
}

#[inline(always)]
fn remove_checked(ensure: &mut Vec<usize>, sq: usize, check: &Move) {
    let shift = if check.flags.bishop_move {
        let check_shift = check.from.abs_diff(check.to);
        if check_shift % 9 == 0 {
            9
        } else {
            7
        }
    } else if check.flags.rook_move {
        let check_shift = check.from.abs_diff(check.to);
        if check_shift < 8 {
            1
        } else {
            8
        }
    } else {
        return;
    };
    
    ensure.retain(|&pl| {
        !( (sq >= shift && pl + shift == sq) // sq - shift it should be
        || (sq + shift <= 63 && pl == sq + shift) )
    });
}
