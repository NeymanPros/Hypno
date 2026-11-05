use crate::board::Board;
use crate::moves::{Move, Flags};

mod board;
mod movegen;
mod moves;

fn main() {
    let mut board = Board::default();
    println!("{}", board);
    loop {
        let mut input = String::default();
        board.generate_all_moves();
        println!("Moves: {:?}", board.all_moves);
        std::io::stdin().read_line(&mut input).expect("Worst");
        let cords: Vec<usize> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().expect("Worst"))
            .collect();
        if cords.len() == 2 {
            let mv = Move::new(cords[0], cords[1], Flags::default());
            if let Some(real_move) = board.all_moves.iter().find(|&real| *real == mv) {
                board.make_move(real_move.clone());
                println!("{}", board);
            } else {
                println!("Invalid move");
            }
        } else {
            println!("Bad amount of numbers")
        }
    }
}