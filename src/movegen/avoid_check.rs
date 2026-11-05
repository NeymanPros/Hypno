use crate::Board;

impl Board {
    pub(super) fn avoid_checks(&mut self) {
        let king = if self.turn { self.white_king } else { self.black_king };
        if self.checks.len() == 1 {
            self.generate_king(king);
            let attack = self.checks[0].clone();
            self.enter_line(&attack, false);
            let mut eating = self.go_square(attack.from, true, self.turn, false);
            self.all_moves.append(&mut eating);
        }
        // Does not intercept with brq.
        self.generate_king(king);
    }
}