use crate::{Board, Move};

impl Board {
    pub(super) fn make_move(&mut self, mv: Move) {
        // Pre-moved changes
        if self.passant {
            self.remove_passants();
        }
        
        if mv.flags.castle {
            self.make_castle(mv);
            return;
        }
        
        // Make a move
        self.set_square(mv.to, self.get_square(mv.from)); 
        self.set_square(mv.from, 0);
        
        self.fifty_rule += 1;
        
        // Post-move chagnes
        if mv.flags.pawn_move {
            self.fifty_rule = 0;
            if mv.flags.passant {
                self.passant_eat(&mv);
            }
            if (mv.to >> 3) == 7 || (mv.to >> 3) == 0 {
                self.make_queening(&mv);
            } else {  
                self.add_passants(&mv);
            }
        }
        
        if mv.flags.eat {
            self.fifty_rule = 0;
        }
        if mv.flags.king_move {
            if self.turn {
                self.white_king = mv.to;
            } else {
                self.black_king = mv.to;
            }
            self.king_no_castle(&mv);
        }
        if mv.flags.rook_move && !mv.flags.bishop_move {
            self.rook_no_castle(&mv);
        }
        
        // Change turn in the end.
        self.turn = !self.turn;
    }
    
    #[inline(always)]
    fn remove_passants(&mut self) {
        let range = if self.turn { 32..=39 } else { 24..=31 };
        for to_check in range {
            let sq = self.get_square(to_check);
            let maybe_pawn = sq & 0b00_1111_11; // Both colors
            if maybe_pawn == 0b00_0001_10 || maybe_pawn == 0b00_0001_01 || maybe_pawn == 0b00_0001_11 {
                self.set_square(to_check, 0b11_0001_00 & sq);
            }
        }
        self.passant = false
    }
    
    #[inline(always)]
    fn make_castle(&mut self, mv: Move) {
        if self.turn {
            self.white_king = mv.to;
        } else {
            self.black_king = mv.to;
        }
        self.fifty_rule += 1;
        self.turn = !self.turn;
        match mv.to {
            2 => {
                self.set_square(0, 0);
                self.set_square(1, 0);                
                self.set_square(2, 0b10_1000_00);
                self.set_square(3, 0b10_1100_00); 
                self.set_square(4, 0);
            },
            6 => {
                self.set_square(4, 0);
                self.set_square(5, 0b10_1100_00);
                self.set_square(6, 0b10_1000_00);
                self.set_square(7, 0); 
            },
            58 => {
                self.set_square(56, 0);
                self.set_square(57, 0);
                self.set_square(58, 0b01_1000_00);
                self.set_square(59, 0b01_1100_00);
                self.set_square(60, 0);
            },
            62 => {
                self.set_square(60, 0);
                self.set_square(61, 0b01_1100_00);
                self.set_square(62, 0b01_1000_00);
                self.set_square(63, 0);
            },
            _ => panic!("Bad castle!")
        };
    }
    
    #[inline(always)]
    fn passant_eat(&mut self, mv: &Move) {
        let change = mv.from as i8 - mv.to as i8;
        let place = match change {
            9 | -7 => mv.from - 1,
            7 | -9 => mv.from + 1,
            _ => panic!("Passant error!"),
        };
        self.set_square(place, 0);
    }
    
    /// Gets color and deletes pawn bit.
    #[inline(always)]
    fn make_queening(&mut self, mv: &Move) {
        let was = self.get_square(mv.to);
        if mv.flags.bishop_move && mv.flags.rook_move {
            self.set_square(mv.to, 0b00_1111_00 ^ was);
        } else if mv.flags.bishop_move {
            self.set_square(mv.to, 0b00_0111_00 ^ was);
        } else if mv.flags.rook_move{
            self.set_square(mv.to, 0b00_1101_00 ^ was);
        } else if mv.flags.knight_move {
            self.set_square(mv.to, 0b00_0101_00 ^ was);
        } else {
            panic!("Impossible queening");
        }
    }
    
    #[inline(always)]
    fn add_passants(&mut self, mv: &Move) {
        let rows = if self.turn {
            (1, 3)
        } else {
            (6, 4)
        };
        
        // Find 2 square moves.
        if (mv.from >> 3) == rows.0 {
            if (mv.to >> 3) == rows.1 {
                let is_pawn = |board: &Self, sq: usize| {
                    (board.get_square(sq) & 0b00_1111_00) == 0b00_0001_00
                };
                if is_pawn(&self, mv.to - 1) && self.is_diff_colors(mv.to - 1, mv.to) && (mv.to - 1) >> 3 == rows.1 {
                    self.set_square(mv.to - 1, self.get_square(mv.to - 1) | 0b00_0000_10); // reverse when granting passants.
                    self.passant = true;
                }
                if is_pawn(&self, mv.to + 1) && self.is_diff_colors(mv.to + 1, mv.to) && (mv.to + 1) >> 3 == rows.1 {
                    self.set_square(mv.to + 1, self.get_square(mv.to + 1) | 0b00_0000_01); // reverse for granting passants.
                    self.passant = true;
                }
            }
        }
    }
    
    #[inline(always)]
    fn king_no_castle(&mut self, mv: &Move) {
        if self.turn && mv.from == 4 {
            self.set_square(mv.to, 0b10_1000_00);
        } else if !self.turn && mv.from == 60 {
            self.set_square(mv.to, 0b01_1000_00);
        }
    }
    
    #[inline(always)]
    fn rook_no_castle(&mut self, mv: &Move) {
        if self.turn && (mv.from == 0 || mv.from == 7) {
            self.set_square(mv.to, 0b10_1100_00);
        } else if !self.turn && (mv.from == 56 || mv.from == 63) {
            self.set_square(mv.to, 0b01_1100_00);
        }
    }
}
