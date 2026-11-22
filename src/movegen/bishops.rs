use crate::{Board, Flags, Move};

impl Board {
    pub fn generate_bishops(&mut self, sq: usize) {
         let maybe_pin = self.xrays.iter().find(|pin| {
            sq == pin.to
        });
        if let Some(pin) = maybe_pin {
            return self.gen_bishops_pinned(sq, pin.clone());
        }
        
        for add in [-9i8, -7, 7, 9] {
            let mut to_check = sq as i8 + add;
            let cond = |int: i8, sq_i: i8|{
                let shift = (
                    (sq_i >> 3).abs_diff(int >> 3), 
                    (sq_i % 8).abs_diff(int % 8)
                );
                int >= 0 && int <= 63 && shift.0 == shift.1
            };
        
            let flags = Flags {
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
    
    fn gen_bishops_pinned(&mut self, sq: usize, pin: Move) {
        let king = self.get_king(self.turn);
        if pin.flags.bishop_move {
            let from = pin.from as i8;
            let king_i8 = king as i8;
            let shift = (((king_i8 >> 3) - (from >> 3)).signum(), ((king_i8 % 8) - (from % 8)).signum());
            let add = match shift {
                (1, 1) | (-1, -1) => 9,
                (-1, 1) | (1, -1) => 7,
                _ => return
            };
            
            let min = pin.from.min(king) + add;
            let max = pin.from.max(king);
            let flags = Flags {
                bishop_move: true,
                ..Flags::default()
            };
            
            let mut pl = min;
            while pl < max {
                if pl == sq {
                    pl += add;
                    continue;
                }
                self.all_moves.push(Move::new(sq, pl, flags.clone()));
                pl += add;
            }
            self.all_moves.push(Move::new(sq, pin.from, flags.with_eat()));
        }
    }
}
