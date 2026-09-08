use std::fmt::{Display, Formatter};
use crate::moves::Move;

/// First 3 bits - type of piece, 4th is color
///
#[derive(Debug)]
pub struct Board {
    board: [u8; 64],
    pub turn: bool,
    passant: bool,
    pub all_moves: Vec<Move>,
    //castle: [bool; 4]
    //total_pieces: u8,
    //col_pawns: u8
}

impl Board {
    #[inline(always)]
    pub fn get_square(&self, place: u8) -> u8 {
        self.board[place as usize]
    }
    #[inline(always)]
    fn set_square(&mut self, place: u8, val: u8) {
        self.board[place as usize] = val
    }

    #[inline(always)]
    pub fn is_piece(&self, place: u8) -> bool {
        let square = self.get_square(place);
        square >= 0b0010_0000
    }
    #[inline(always)]
    pub fn is_white_piece(&self, place: u8) -> bool {
        let square = self.get_square(place);
        square >= 0b0011_0000
    }
    #[inline(always)]
    pub fn is_black_piece(&self, place: u8) -> bool {
        let square = self.get_square(place);
        (square & 0b0001_0000) == 0 && square >= 0b0010_0000
    }
    #[inline(always)]
    /// Assumes both are confirmed to be pieces.
    pub fn is_diff_colors(&self, first: u8, second: u8) -> bool{
        let one = self.get_square(first);
        let two = self.get_square(second);
        ((one ^ two) & 0b0001_0000) != 0
    }

    pub fn make_move(&mut self, mv: Move) {
        if self.passant { // remove passants
            let start = (3 + self.turn as u8) * 8;
            for i in start..=(start + 7) {
                let square = self.get_square(i) & 0b1110_1100;
                if square == 0b0010_1100 ||
                    square == 0b0010_0100 ||
                    square == 0b0010_1000 {
                    self.set_square(i, 0b0011_0011 & self.get_square(i));
                    println!("Removed passant")
                }
            }
            self.passant = false
        }
        if mv.flags.king_move { // castle catch
            return
        }
        if mv.flags.eat && mv.flags.pawn_move { // passant eat
            if !self.is_piece(mv.to) {
                let change = mv.from as i8 - mv.to as i8;
                let place = match change {
                    -9 => mv.from + 1,
                    -7 => mv.from - 1,
                    7 => mv.to - 1,
                    9 => mv.to + 1,
                    _ => panic!("Passant error!")
                };
                self.set_square(place, 0);
            }
        }
        self.set_square(mv.to, self.get_square(mv.from)); // make a move
        self.set_square(mv.from, 0);

        if mv.flags.pawn_move { // add passants
            if mv.to / 8 == 4 - self.turn as u8 {
                if mv.from / 8 == 6 - (self.turn as u8 * 5) { // find 2 square moves
                    println!("Adding passants");
                    if self.is_white_piece(mv.to) {
                        if (self.get_square(mv.to - 1) & 0b1111_0000) == 0b0010_0000 {
                            self.set_square(mv.to - 1, self.get_square(mv.to - 1) | 0b0000_0100); // eatable for -7
                            self.passant = true;
                        }
                        if (self.get_square(mv.to + 1) & 0b1111_0000) == 0b0010_0000 {
                            self.set_square(mv.to + 1, self.get_square(mv.to + 1) | 0b0000_1000); // eatable for -9
                            self.passant = true;
                        }
                    }
                    else {
                        if (self.get_square(mv.to - 1) & 0b1111_0000) == 0b0011_0000 {
                            self.set_square(mv.to - 1, self.get_square(mv.to - 1) | 0b0000_1000); // eatable for +9
                            self.passant = true;
                        }
                        if (self.get_square(mv.to + 1) & 0b1111_0000) == 0b0011_0000 {
                            self.set_square(mv.to + 1, self.get_square(mv.to + 1) | 0b0000_0100); // eatable for +7
                            self.passant = true;
                        }
                    }
                }
            }
        }
        self.turn = !self.turn;
    }

    pub fn from_fen(fen: &'static str) -> Option<Self> {
        None
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: std::array::from_fn(|i| match i {
                0 | 7 => 0b1101_1000,
                63 | 56 => 0b1100_1000,
                1 | 6 => 0b0101_0000,
                62 | 57 => 0b0100_0000,
                2 | 5 => 0b0111_0000,
                61 | 58 => 0b0110_0000,
                4 => 0b1111_0000,
                60 => 0b1110_0000,
                3 => 0b1011_1000,
                59 => 0b1010_1000,
                8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => 0b0011_0000,
                55 | 54 | 53 | 52 | 51 | 50 | 49 | 48 => 0b0010_0000,
                _ => 0,
            }),
            turn: true,
            passant: false,
            all_moves: vec![],
            //total_pieces: 0,
            //col_pawns: 0
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::default();
        for p in 0..=63 {
            if p % 8 == 0 {
                out.push_str("/\n");
            }
            let square = self.get_square(p);
            let c = match square & 0b1111_0000 {
                0b1111_0000 => 'Q',
                0b1110_0000 => 'q',
                0b1011_0000 => 'K',
                0b1010_0000 => 'k',
                0b1101_0000 => 'R',
                0b1100_0000 => 'r',
                0b0111_0000 => 'B',
                0b0110_0000 => 'b',
                0b0101_0000 => 'N',
                0b0100_0000 => 'n',
                0b0011_0000 => 'P',
                0b0010_0000 => 'p',
                _ => '0'
            };
            out.push(c);
        }
        f.write_str(out.as_str())
    }
}
