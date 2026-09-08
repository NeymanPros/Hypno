use crate::board::Board;
use crate::moves::{Flags, Move};

impl Board {
    pub fn generate_all_moves(&mut self) {
        let checks = self.king_check();
        if checks.len() == 0 {
            return
        }
        let xrays = self.king_xray();
        for sq in 0..=63 {
            if self.is_white_piece(sq) == self.turn {
                let square = self.get_square(sq);
                match square & 0b1110_0000 {
                    0b0010_0000 => self.generate_pawns(sq, &xrays),
                    0b1100_0000 => self.generate_rooks(sq, &xrays),
                    _ => {}
                }
            }
        }
    }

    /// Direct attacks. Use 1100 for rooks and 0110 for bishops
    fn king_check(&self) -> Vec<Move> {
        let mut checks = vec![];

        checks
    }

    /// X-rays through only 1 piece.
    fn king_xray(&self, ) -> Vec<Move> {
        let mut xrays = vec![];

        xrays
    }

    fn generate_pawns(&mut self, sq: u8, xrays: &Vec<Move>) {

    }
    fn generate_rooks(&mut self, sq: u8, xrays: &Vec<Move>) {
        let mut flags = Flags::default();
        flags.rook_move = true;
        
        for xray in xrays {
            self.piece_in_xray(sq, &xray);
        }
        let mut sq_mut = sq;
        if sq_mut + 8 < 64 {
            while(sq_mut < 8) {
                if !self.is_piece(sq_mut) {
                    self.all_moves.push(Move::new(sq, sq_mut, flags.clone()))
                } else if self.is_diff_colors(sq_mut, sq) {
                    let mut temp_flags = flags.clone();
                    temp_flags.eat = true;
                    self.all_moves.push(Move::new(sq, sq_mut, temp_flags))
                } else {
                    break
                }
            }
        }
        sq_mut = sq;
        if sq_mut - 8 > 0 {

        }
    }
    fn piece_in_xray(&self, sq: u8, xray: &Move) -> bool {
        false
    }

    fn enter_straight(&mut self, from: u8, to: u8) -> Vec<Move> {
        vec![]
    }
    fn enter_diagonal(&mut self, from: u8, to: u8) -> Vec<Move> {
        vec![]
    }
}

// color == turn
