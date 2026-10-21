use crate::Board;
use crate::moves::{Flags, Move};

impl Board {
    pub fn generate_rooks(&mut self, sq: usize) {
        for i in 0..self.xrays.len() {
            if self.piece_in_xray(sq, &self.xrays[i]) {
                return self.gen_rooks_pinned(sq, self.xrays[i].clone());
            }
        }
        
        for add in [-8i8, -1, 1, 8] {
            let mut to_check = sq as i8 + add;
        
            let flags = Flags{
                rook_move: true,
                ..Flags::default()
            };
            
            let cond = match add {
                -1 | 1 => |int: i8, sq_i: i8| {sq_i >> 3 == int >> 3},
                _ => |int: i8, _sq_i: i8| {int >= 0 && int <= 63}
            };
            
            while cond(to_check, sq as i8) {
                if self.add_exit(sq, to_check as usize, flags.clone()) {
                    break;
                }
                to_check += add;
            }
        }
    }

    #[inline(always)]
    fn gen_rooks_pinned(&mut self, sq: usize, pin: Move) {
        if pin.flags.rook_move {
            let add = if pin.from >> 8 == pin.to >> 8 {
                1
            } else if pin.from % 8 == pin.to % 8 {
                8
            } else {
                return
            };
            let min = pin.from.min(pin.to) + add;
            let max = pin.from.max(pin.to);
            let flags = Flags {
                rook_move: true,
                ..Flags::default()
            };
            let mut pl = min;
            while pl < max {
                if pl == sq {
                    continue;
                }
                self.all_moves.push(Move::new(sq, pl, flags.clone()));
                pl += add;
            }
            let _ = self.add_exit(sq, pin.from, flags);
            }
    }
}
