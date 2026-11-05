use crate::Board;
use crate::moves::{Move, Flags};

impl Board {
    #[inline(always)]
    pub(super) fn get_king(&self, white: bool) -> usize {
        if white { self.white_king } else { self.black_king }
    }
    
    pub(super) fn generate_king(&mut self, sq: usize) {
        let all_kings = self.gen_all_kings(sq as i8);
        for to_check in all_kings {
            
        }
    }
    
    #[inline(always)]
    fn gen_all_kings(&self, sq: i8) -> Vec<usize> {
        let pos= (sq >> 3, sq % 8);
        let dirs = match pos {
            (0, 0) => vec![1i8, 8, 9],
            (0, 7) => vec![-1, 7, 8],
            (7, 0) => vec![1, -8, -7],
            (7, 7) => vec![-1, -8, -9],
            (0, _) => vec![-1, 1, 7, 8, 9],
            (7, _) => vec![-1, 1, -7, -8, -9],
            (_, 0) => vec![1, -8, 8, -7, 9],
            (_, 7) => vec![-1, -8, -8, -9, 7],
            _ => vec![-9, -8, -7, -1, 1, 7, 8, 9]
        };
        
        dirs.into_iter().map(|add| {
            (sq + add) as usize
        }).collect()
    }
}
