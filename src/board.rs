use crate::moves::Move;
use std::fmt::{Display, Formatter};

/// First 3 bits - type of piece, 4th is color
///
#[derive(Debug)]
pub struct Board {
    board: [u8; 64],
    pub turn: bool,
    pub passant: bool,
    pub all_moves: Vec<Move>,
    pub white_king: usize,
    pub black_king: usize,
    pub checks: Vec<Move>,
    pub xrays: Vec<Move>,
    //castle: [bool; 4]
    //total_pieces: u8,
    //col_pawns: u8
}

impl Board {
    #[inline(always)]
    pub fn get_square(&self, place: usize) -> u8 {
        self.board[place]
    }
    #[inline(always)]
    fn set_square(&mut self, place: usize, val: u8) {
        self.board[place] = val
    }

    #[inline(always)]
    pub fn is_piece(&self, place: usize) -> bool {
        let square = self.get_square(place);
        square >= 0b0010_0000
    }
    #[inline(always)]
    pub fn is_white_piece(&self, place: usize) -> bool {
        let square = self.get_square(place);
        (square >= 0b0011_0000) && (square & 0b0001_0000 == 0b0001_0000)
    }
    #[inline(always)]
    pub fn is_black_piece(&self, place: usize) -> bool {
        let square = self.get_square(place);
        (square >= 0b0010_0000) && (square & 0b0001_0000 == 0)
    }
    #[inline(always)]
    /// Assumes both are confirmed to be pieces.
    pub fn is_diff_colors(&self, first: usize, second: usize) -> bool {
        let one = self.get_square(first);
        let two = self.get_square(second);
        ((one ^ two) & 0b0001_0000) != 0
    }

    pub fn make_move(&mut self, mv: Move) {
        if self.passant {
            // remove passants
            let row = (3 + self.turn as usize) * 8;
            for letter in 0..8 {
                let place = letter * 8 + row;
                let sq = self.get_square(place);
                let square = sq & 0b1110_1100; // Both colors.
                if square == 0b0010_1100 || square == 0b0010_0100 || square == 0b0010_1000 {
                    self.set_square(place, 0b0011_0011 & sq);
                    println!("Removed passant")
                }
            }
            self.passant = false
        }
        if mv.flags.king_move {
            // castle catch
            return;
        }
        if mv.flags.eat && mv.flags.pawn_move {
            // passant eat
            if !self.is_piece(mv.to) {
                let change = (
                    mv.from as i8 >> 3 - mv.to as i8 >> 3,
                    mv.from as i8 % 8 - mv.to as i8 % 8,
                );
                let place = match change {
                    (-1, 1) | (1, 1) => mv.from + 1,
                    (-1, -1) | (1, -1) => mv.from - 1,
                    _ => panic!("Passant error!"),
                };
                self.set_square(place, 0);
            }
        }
        self.set_square(mv.to, self.get_square(mv.from)); // make a move
        self.set_square(mv.from, 0);

        if mv.flags.pawn_move {
            // queening
            if mv.to == 7 || mv.to == 0 {
                let was = self.get_square(mv.to);
                if mv.flags.bishop_move && mv.flags.rook_move {
                    self.set_square(mv.to, 0b1110_0000 & was);
                } else if mv.flags.bishop_move {
                    self.set_square(mv.to, 0b0110_0000 & was);
                } else if mv.flags.rook_move{
                    self.set_square(mv.to, 0b1100_0000 & was);
                } else if mv.flags.knight_move {
                    self.set_square(mv.to, 0b0100_0000 & was);
                }
            }
            // add passants
            if mv.to / 8 == 4 - self.turn as usize {
                if mv.from / 8 == 6 - (self.turn as usize * 5) {
                    // find 2 square moves
                    if self.is_white_piece(mv.to) {
                        if (self.get_square(mv.to - 1) & 0b1111_0000) == 0b0010_0000 {
                            self.set_square(mv.to - 1, self.get_square(mv.to - 1) | 0b0000_0100); // eatable for -7
                            self.passant = true;
                        }
                        if (self.get_square(mv.to + 1) & 0b1111_0000) == 0b0010_0000 {
                            self.set_square(mv.to + 1, self.get_square(mv.to + 1) | 0b0000_1000); // eatable for -9
                            self.passant = true;
                        }
                    } else {
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
                24 | 25 | 10 | 11 | 12 | 13 | 14 | 15 => 0b0011_0000,
                55 | 54 | 53 | 52 | 51 | 50 | 33 | 40 => 0b0010_0000,
                _ => 0,
            }),
            turn: true,
            passant: false,
            all_moves: vec![],
            white_king: 3,
            black_king: 59,
            xrays: vec![],
            checks: vec![],
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
                _ => '0',
            };
            out.push(c);
        }
        f.write_str(out.as_str())
    }
}
