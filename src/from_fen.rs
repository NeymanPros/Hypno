#[inline(always)]
pub(super) fn get_board(fen: &'static str) ->([u8; 64], usize, usize) {
    let rows = fen.split('/').rev();
    let mut white_king = 0;
    let mut black_king = 0;
    let mut board = [0; 64];
    let mut num = 0;
    for row in rows {
        for char in row.chars() {
            board[num] = match char {
                'R' => 0b10_1100_00, // White rook
                'r' => 0b01_1100_00, // Black rook
                'N' => 0b10_0100_00, //White knight
                'n' => 0b01_0100_00, // Black knight
                'B' => 0b10_0110_00, // White bishop
                'b' => 0b01_0110_00, // Black bishop
                'Q'=> 0b10_1110_00, // White queen
                'q' => 0b01_1110_00, // Black queen
                'K'=> {
                    white_king = num; // White king
                    0b10_1000_00
                }, 
                'k' => {
                    black_king = num; // Black king
                    0b01_1000_00
                }, 
                'P' => 0b10_0001_00, // White pawn
                'p' => 0b01_0001_00, // Black pawn
                _ => {
                    num += char as usize - '1' as usize;
                    0
                },
            };
            num += 1;
        }
    }
    (board, white_king, black_king)
}

#[inline(always)]
pub(super) fn get_turn(fen: &'static str) -> bool {
    if fen == "w" {
        true
    } else if fen == "b" {
        false
    } else {
        panic!("Incorrect fen")
    }
}

#[inline(always)]
pub(super) fn add_castles(board: &mut [u8; 64], fen: &'static str) {
    if fen == "-" {
        return
    }
    for char in fen.chars() {
        let rook_sq = match char {
            'K' => {
                board[4] = board[4] | 0b00_0000_10;
                0
            },
            'Q' => {
                board[4] = board[4] | 0b00_0000_10;
                7
            },
            'k' => {
                board[60] = board[60] | 0b00_0000_10;
                56
            },
            'q' => {
                board[60] = board[60] | 0b00_0000_10;
                63
            },
            _ => panic!("Bad castle fen")
        };
        board[rook_sq] = board[rook_sq] | 0b00_0000_10;
    }
}

#[inline(always)]
pub(super) fn get_passant(board: &mut [u8; 64], fen: &'static str) -> bool {
    if fen == "-" {
        false
    } else {
        let msg = "Incorrect fen passant";
        let letter = fen.chars().nth(0).expect(msg);
        let number = fen.chars().nth(1).expect(msg);
        let number = if number == '6' {
            32
        } else if number == '3' {
            24
        } else {
            panic!("{msg}");
        };
        
        match letter {
            'a' => {
                board[number + 1] |= 0b00_0000_01;
            },
            'h' => {
                board[number + 6] |= 0b00_0000_10;                
            },
            _ => {
                let pawn = if number == 40 { 0b10_0001_00 } else { 0b01_0001_00 };
                if board[number + 1] == pawn {
                    board[number + 1] |= 0b00_0000_10;
                }
                if board[number - 1] == pawn {
                    board[number - 1] |= 0b00_0000_01
                }
            }
        };
        true
    }
}

#[inline(always)]
/// Returns fifty rule and number of a move.
pub(super) fn get_rest(mut fen: std::str::Split<char>) -> (u8, u16) {
    let msg = "Incorrect fen final";
    let fifty = fen.next()
        .expect(msg)
        .parse::<u8>()
        .expect(msg);
    let num = fen
        .next()
        .expect(msg)
        .parse::<u16>()
        .expect(msg);
    (fifty, num)
}
