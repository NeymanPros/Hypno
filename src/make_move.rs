use crate::{Board, Move};

impl Board {
    pub(super) fn make_move(&mut self, mv: Move) {
        // Pre-moved changes
        if self.passant {
	    println!("Passant was true");
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
            if mv.to == 7 || mv.to == 0 {
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
        self.turn = !self.turn;
    }
    
    #[inline(always)]
    fn remove_passants(&mut self) {
        let row = (3 + self.turn as usize) << 3;
        for letter in 0..8 {
            let place = row + letter;
            let sq = self.get_square(place);
            let square = sq & 0b00_1111_11; // Both colors
            if square == 0b00_0001_11 || square == 0b00_0001_10 || square == 0b00_0001_01 {
                self.set_square(place, 0b11_0001_00 & sq);
            }
        }
        self.passant = false
    }
    
    #[inline(always)]
    fn make_castle(&mut self, mv: Move) {
        self.turn = !self.turn;
        self.fifty_rule += 1;
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
        let change = (
            (mv.from as i8 >> 3) - (mv.to as i8 >> 3),
            mv.from as i8 % 8 - mv.to as i8 % 8,
        );
        // Shuffled because it checks the direction.
        let place = match change {
            (-1, -1) | (1, -1) => mv.from + 1,
            (-1, 1) | (1, 1) => mv.from - 1,
            _ => panic!("Passant error!"),
        };
        self.set_square(place, 0);
    }
    
    #[inline(always)]
    fn make_queening(&mut self, mv: &Move) {
        let was = self.get_square(mv.to);
        if mv.flags.bishop_move && mv.flags.rook_move {
            self.set_square(mv.to, 0b00_1110_00 | was);
        } else if mv.flags.bishop_move {
            self.set_square(mv.to, 0b00_0110_00 | was);
        } else if mv.flags.rook_move{
            self.set_square(mv.to, 0b00_1100_00 | was);
        } else if mv.flags.knight_move {
            self.set_square(mv.to, 0b00_0100_00 | was);
        }
    }
    
    #[inline(always)]
    fn add_passants(&mut self, mv: &Move) {
        let shift = if self.turn {
            (1, 3)
        } else {
            (6, 4)
        };
        
        // Find 2 square moves.
        if mv.from >> 3 == shift.0 {
            if mv.to >> 3 == shift.1 {
                let is_pawn = |board: &Self, sq: usize| {
                    (board.get_square(sq) & 0b00_1111_00) == 0b00_0001_00
                };
                if is_pawn(&self, mv.to - 1) && self.is_diff_colors(mv.to - 1, mv.to) && (mv.to - 1) >> 3 == shift.1 {
                    self.set_square(mv.to - 1, self.get_square(mv.to - 1) | 0b00_0000_01); // -1 == _01
                    self.passant = true;
                }
                if is_pawn(&self, mv.to + 1) && self.is_diff_colors(mv.to + 1, mv.to) && (mv.to + 1) >> 3 == shift.1 {
                    self.set_square(mv.to + 1, self.get_square(mv.to + 1) | 0b00_0000_10); // +1 == _10
                    self.passant = true;
                }
            }
        }
    }
    
    #[inline(always)]
    fn king_no_castle(&mut self, mv: &Move) {
        if self.turn && mv.from == 3 {
            self.set_square(mv.to, 0b10_1000_00)
        } else if !self.turn && mv.from == 59 {
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
