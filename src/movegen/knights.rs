use crate::board::Board;
use crate::moves::{Move, Flags};

impl Board {
    pub fn generate_knights(&mut self, sq: usize) {
        for xray in &self.xrays {
            if xray.to == sq {
                return
            }
        }
        let knights = self.return_knights(sq);
        for to_check in knights.into_iter() {
            if self.is_piece(to_check) {
                if self.is_diff_colors(to_check, sq) {
                    let flags = Flags{
                    knight_move: true,
                    eat: true,
                    ..Flags::default()
                };
                self.all_moves.push(Move::new(sq, to_check, flags))
                }
            } else {
                let flags = Flags{
                    knight_move: true,
                    ..Flags::default()
                };
                self.all_moves.push(Move::new( sq, to_check, flags))
            }
        }
    }
    
    /// Knight moves from specified square. Doesn't ensure if the target is occupied.
    pub fn return_knights(&self, sq: usize) -> Vec<usize> {
        let mut adding = vec![];
        for num in [-17i8, -15, -10, -6, 6, 10, 15, 17] {
            let to_check = sq as i8 + num;
            if to_check < 0 || to_check > 63 {
                continue;
            }
            let shift = (((to_check >> 3) - (sq as i8 >> 3)).abs(), (( to_check % 8) - (sq as i8 % 8)).abs());
            let to_check = to_check as usize;
            match shift {
                (1, 2) | (2, 1) => {
                    adding.push(to_check);
                },
                _ => continue
            }
        }
            return adding;
    }
}
