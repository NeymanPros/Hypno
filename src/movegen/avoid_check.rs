use crate::{Board, Move, Flags};

impl Board {
    pub(super) fn avoid_checks(&mut self) {
        let king = if self.turn { self.white_king } else { self.black_king };
        if self.checks.len() == 1 {
            let attack = self.checks[0].clone();
            self.enter_line(&attack, false);
            let mut eating = self.go_square(attack.from, true, self.turn, false, false);
            self.all_moves.append(&mut eating);
            self.passant_uncheck(attack);
        }
        // Does not intercept with brq.
        self.generate_king(king);
    }
    
    /// Eat checking pawn with passant.
    #[inline(always)]
    fn passant_uncheck(&mut self, attack: Move, ) {
        if attack.flags.pawn_move && self.passant {
            let row = if self.turn {
                4
            } else {
                3
            };
            if (attack.from >> 3) == row {
                for (to_check, pawn) in [(attack.from - 1, 0b00_0001_10), (attack.from + 1, 0b00_0001_01)] {
                    // Swapped _10 and _01
                    let sq = self.get_square(to_check);
                    // If position before is possible.
                    if (sq & 0b00_1111_11) == pawn {
                        if self.xrays.iter().all(|xray| to_check != xray.to) {
                            let to = if self.turn { attack.from + 8 } else { attack.from - 8 };
                            self.all_moves.push(Move::new (to_check, to, Flags {
                                eat: true, 
                                passant: true, 
                                pawn_move: true,
                                ..Flags::default()
                            }
                            ))
                        }
                    }
                }
            }
        }
    }
}