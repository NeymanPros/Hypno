use crate::board::Board;
use crate::moves::Move;

mod board;
mod moves;
mod movegen;

fn main() {
    let mut board = Board::default();
    println!("{}", board);
    loop {
        let mut input = String::default();
        std::io::stdin().read_line(&mut input).expect("Worst");
        let cords: Vec<u8> = input.trim().split(' ').map(|x| x.parse::<u8>().expect("Worst")).collect();
        if cords.len() == 4 {
            let mv = Move::with_flags(&board, cords[0] * 8 + cords[1], cords[2] * 8 + cords[3]);
            if mv.is_none() {
                println!("Wrong move")
            } else {
                board.make_move(mv.unwrap());
                println!("{}", board)
            }
        } else {
            println!("Invalid pieces");
        }
        
        board.generate_all_moves();
        println!("Moves: {:?}", board.all_moves)
    }
}
