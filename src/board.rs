use crate::Move;
use std::fmt::{Display, Formatter};
use crate::from_fen::*;

#[derive(Debug, Clone)]
pub struct Board {
    /// First 2 bits - color, 4 bits - pieces, 2 last flags
    board: [u8; 64],
    pub turn: bool,
    pub passant: bool,
    pub fifty_rule: u8,
    
    pub white_king: usize,
    pub black_king: usize,
    pub all_moves: Vec<Move>,
    pub checks: Vec<Move>,
    pub xrays: Vec<Move>
}

impl Board {
    #[inline(always)]
    pub fn get_square(&self, place: usize) -> u8 {
        self.board[place]
    }
    #[inline(always)]
    pub(crate) fn set_square(&mut self, place: usize, val: u8) {
        self.board[place] = val
    }

    #[inline(always)]
    pub fn is_piece(&self, place: usize) -> bool {
        let square = self.get_square(place);
        square >= 0b01_0000_00
    }
    #[inline(always)]
    pub fn is_white_piece(&self, place: usize) -> bool {
        let square = self.get_square(place);
        square > 0b10_0000_00
    }
    #[inline(always)]
    pub fn is_black_piece(&self, place: usize) -> bool {
        let square = self.get_square(place);
        (square & 0b01_0000_00) == 0b01_0000_00
    }
    #[inline(always)]
    /// True only if there are 2 different colored pieces.
    pub fn is_diff_colors(&self, first: usize, second: usize) -> bool {
        let one = self.get_square(first);
        let two = self.get_square(second);
        (one | two) >= 0b11_0000_00
    }
}

impl Board {
    pub fn from_fen(fen: &'static str) -> Self {
        let mut split_fen = fen.split(' ');
        let msg = "Incorrect fen";
        let (mut board, white_king, black_king) = get_board(split_fen.next().expect(msg));
        let turn = get_turn(split_fen.next().expect(msg));
        add_castles(&mut board, split_fen.next().expect(msg));
        let passant = get_passant(&mut board, split_fen.next().expect(msg));
        let (fifty_rule, _)= get_rest(split_fen);
        
        Self {
            board,
            turn, 
            fifty_rule,
            passant,
            white_king,
            black_king,
            all_moves: vec![],
            checks: vec![],
            xrays: vec![],
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::default();
        for add in [56usize, 48, 40, 32, 24, 16, 8, 0] {
            out.push('\n');
            for p in add..=(add+7) {
                let square = self.get_square(p);
                let c = match square & 0b11_1111_00 {
                    0b10_1110_00 => 'Q',
                    0b01_1110_00 => 'q',
                    0b10_1000_00 => 'K',
                    0b01_1000_00 => 'k',
                    0b10_1100_00 => 'R',
                    0b01_1100_00 => 'r',
                    0b10_0110_00 => 'B',
                    0b01_0110_00 => 'b',
                    0b10_0100_00 => 'N',
                    0b01_0100_00 => 'n',
                    0b10_0001_00 => 'P',
                    0b01_0001_00 => 'p',
                    _ => '.',
                };
                out.push(c);
            }
        }
        f.write_str(out.as_str())
    }
}
