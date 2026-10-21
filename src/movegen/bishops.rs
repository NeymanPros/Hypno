use crate::board::Board;
use crate::moves::{Flags, Move};

impl Board {
    pub fn generate_moves(&mut self, sq: usize) {
         for i in 0..self.xrays.len() {
            if self.piece_in_xray(sq, &self.xrays[i]) {
                return self.gen_pinned(sq, self.xrays[i].clone());
            }
        }
        
        for add in [-9i8, -7, 7, 9] {
            let mut to_check = sq as i8;
            let cond = |int: i8, sq_i: i8|{
            let shift = (((sq_i >> 3) - (int >> 3)).abs(), ((sq_i % 8) - (int % 8)).abs());
                int >= 0 && int <= 63 && shift.0 == shift.1
            };
        
            let flags = Flags{
                bishop_move: true,
                ..Flags::default()
            };
            
            while cond(to_check, sq as i8) {
                if self.add_exit(sq, to_check as usize, flags.clone()) {
                    break;
                }
                to_check += add;
            }
        }
    }
    
    fn gen_pinned(&mut self, sq: usize, pin: Move) {
        if pin.flags.bishop_move {
            let from = pin.from as i8;
            let to = pin.to as i8;
            let shift = (((to >> 3) - (from >> 3)).signum(), ((to % 8) - (from % 8)).signum());
            let add = match shift {
                (1, 1) | (-1, -1) => 9,
                (-1, 1) | (1, -1) => 7,
                _ => return
            };
            
            let min = pin.from.min(pin.to) + add;
            let max = pin.from.max(pin.to);
            let flags = Flags {
                bishop_move: true,
                ..Flags::default()
            };
            
            for to_check in min..max {
                if to_check == sq {
                    continue;
                }
                self.all_moves.push(Move::new(sq, to_check, flags.clone()));
            }
            
        }
    }
}
