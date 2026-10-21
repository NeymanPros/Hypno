use crate::board::Board;
use crate::moves::{Flags, Move};

impl Board {
    pub fn generate_all_moves(&mut self) {
        self.all_moves = Vec::with_capacity(50);
        self.checks.clear();
        self.xrays.clear();
        
        self.king_check();
        println!("\nChecks: {:?}\nXrays: {:?}\n", self.checks, self.xrays);
        
        for sq in 0..=63 {
            if (self.is_white_piece(sq) & self.turn) || (self.is_black_piece(sq) & !self.turn) {
                let square = self.get_square(sq);
                match square & 0b1110_0000 {
                    //0b0010_0000 => self.generate_pawns(sq),
                    //0b0100_0000 => self.generate_knights(sq),
                    0b0110_0000 => self.generate_moves(sq),
                    0b1100_0000 => self.generate_rooks(sq),
                    0b1110_0000 => {
                        self.generate_rooks(sq);
                        self.generate_moves(sq);
                    },
                    0b0100_0000 => self.generate_king(sq),
                    _ => {}
                }
            }
        }
    }

    #[inline(always)]
    pub fn add_exit(&mut self, sq_from: usize, sq_to: usize, flags: Flags) -> bool {
        if !self.is_piece(sq_to) {
            // Empty square.
            self.all_moves.push(Move::new(sq_from, sq_to, flags));
            false
        } else if self.is_diff_colors(sq_to, sq_from) {
            // Different color piece.
            self.all_moves.push(Move::new(sq_from, sq_to, flags.with_eat()));
            true
        } else {
            // Same color piece.
            true
        }
    }
}
