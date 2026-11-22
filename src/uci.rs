use crate::{Move, Flags};

impl Move {
    pub fn from_uci (uci: String) -> Move {
        let mut iter = uci.as_bytes().into_iter();
        let msg = "Bad UCI";
        let from = (iter.next().expect(msg) - 'a' as u8) + ((iter.next().expect(msg) - '1' as u8) << 3);
        let to = (iter.next().expect(msg) - 'a' as u8) + ((iter.next().expect(msg) - '1' as u8) << 3);
        let from = from as usize;
        let to = to as usize;
        let Some(flags_letter) = iter.next() else {
            return Move::new(from, to, Flags::default())
        };
        let flags = match *flags_letter as char {
            'n' => Flags{
                knight_move: true, ..Flags::default()
            },
            'b' => Flags {
                bishop_move: true, ..Flags::default()
            },
            'r' => Flags {
                rook_move: true, ..Flags::default()
            },
            'q' => Flags {
                rook_move: true, bishop_move: true, 
                ..Flags::default()
            },
            _ => Flags::default()
        };
        Move::new(from, to, flags)
    }
    
    pub fn to_uci (&self) -> String {
        let from_letter = (self.from >> 3) as u8 + 'a'  as u8 - 1;
        let from_num = self.from as u8 % 8 + '1' as u8;
        
        let to_letter = (self.to as u8 >> 3) + 'a'  as u8 - 1;
        let to_num = self.to as u8 % 8 + '1' as u8;
        
        let mut uci = String::from_utf8(vec![from_letter, from_num, to_letter, to_num]).expect("Failed with to_uci");
        if self.flags.pawn_move && (self.to == 7 || self.to == 0) {
            let queen = if self.flags.knight_move {
                'n'
            } else if self.flags.rook_move && self.flags.bishop_move {
                'q'
            } else if self.flags.rook_move {
                'r'
            } else if self.flags.bishop_move {
                'b'
            } else {
                panic!("Bad queening")
            };
            uci.push(queen);
        }
        uci
    }
}