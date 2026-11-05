use crate::Board;

impl Board {
    pub fn king_checks(&mut self) {
        self.checks = vec![];
        let king = if self.turn { self.white_king  } else { self.black_king };
        
        let mut brq = self.brq_occupy_xrays(king, false, false, !self.turn);
        let mut knight = self.knight_occupy(king, !self.turn);
        let mut pawn = self.pawn_eat(king, !self.turn);
        
        self.checks.append(&mut brq);
        self.checks.append(&mut knight);
        self.checks.append(&mut pawn);
    }
}
