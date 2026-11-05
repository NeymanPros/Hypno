use crate::Board;
use crate::moves::Move;

impl Board {
    #[inline(always)]
    /// Pushes into all_moves.
    pub(super) fn enter_line(&mut self, attack: &Move, ignore_xray: bool) {
        let from = attack.from.min(attack.to);
        let to= attack.from.max(attack.to);
        
        let add = if attack.flags.knight_move {
            return
        } else if attack.flags.bishop_move && (to - from) % 9 == 0 {
            9
        } else if attack.flags.bishop_move && (to - from) % 7 == 0 {
            7
        } else if attack.flags.rook_move && to % 8 == from % 8 {
            8
        } else if attack.flags.rook_move && to / 8 == from / 8 {
            1
        } else {
            panic!("Wrong shift in bishop move!")
        };
        
        let mut to_check = from + add;
        while to_check < to {
            let mut moves = self.go_square(to_check, false, self.turn, ignore_xray);
            self.all_moves.append(&mut moves);
            
            to_check += add;
        }
    }
    
    #[inline(always)]
    pub(super) fn go_square(&mut self, sq: usize, eat: bool, target_color: bool, ignore_xray: bool) -> Vec<Move> {
        let mut brq = self.brq_occupy_xrays(sq, false, false, target_color);
        let mut knights = self.knight_occupy(sq, target_color);
        let pawns = if eat {
            self.pawn_eat(sq, target_color)
        } else { 
            self.pawn_go(sq, target_color) 
        };
        
        let mut moves = Vec::with_capacity(8);
        moves.append(&mut brq);
        moves.append(&mut knights);
        append_pawns(&mut moves, pawns);
        if !ignore_xray {
            self.check_xrays(&mut moves)
        }
        moves
    }
    
    #[inline(always)]
    /// Needs to protect from checks only; if pinned move is impossible. Geometry will be impossible.
    fn check_xrays(&self, moves: &mut Vec<Move>) {
        moves
        .retain(|mv|{
            self.xrays.iter().all(|xray| {
                xray.to != mv.from
            })
        });
    }
    
}
#[inline(always)]
fn append_pawns(moves: &mut Vec<Move>, pawns: Vec<Move>) {
    for pawn in pawns.into_iter() {
        crate::movegen::pawns::add_pawn_move(moves, pawn.from, pawn.to, pawn.flags);
    }
}
