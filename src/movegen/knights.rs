use crate::board::Board;
use crate::moves::{Move, Flags};

impl Board {
    pub fn generate_knights(&mut self, sq: usize) {
        let add_what = |board: &Self, sq: usize, to_check: usize| -> Option<Flags> {
            if board.is_piece(to_check) {
                // Is a piece.
                if board.is_diff_colors(sq, to_check) {
                    Some (Flags {
                        knight_move: true,
                        eat: true,
                        ..Flags::default()
                    })
                } else {
                    None
                }
            } else {
                // Not a piece.
                Some(Flags {
                    knight_move: true,
                    ..Flags::default()
                })
            }
        };
        let mut knights = self.return_knights(sq, add_what);
        println!("Knights are: {:?}", knights);
        self.all_moves.append(&mut knights);
    }
    
    pub fn return_knights(&self, sq: usize, add_what: fn(&Self, usize, usize) -> Option<Flags>) -> Vec<Move> {
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
                    if let Some(flags) = add_what(&self, sq, to_check) {
                        adding.push(Move::new(sq, to_check, flags));
                    }
                },
                _ => continue
            }
        }
        return adding;
    }
}
