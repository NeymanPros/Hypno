use crate::board::Board;
use crate::moves::{Move, Flags};

mod board;
mod movegen;
mod moves;
mod make_move;
mod from_fen;
mod uci;
mod perft;

fn main() {
    run_perft();
}

fn run_perft() {
    let mut board = Board::default();
    let moves = perft::perft(&mut board, 5);
    println!("Moves: {}", moves);
    return;
}

fn run_some() {
    let mut board = Board::from_fen("r1bqkbnr/pppppppp/8/3Q4/N1BPP3/8/PPP2PPP/Rn2KBNR w KQkq - 0 1");    
    println!("{}", board);
    loop {
        let mut input = String::default();
        board.generate_all_moves();
        std::io::stdin().read_line(&mut input).expect("Worst");
        let Ok(from) = input.trim().parse::<usize>() else { continue; };
        let moves: Vec<&Move> = board.all_moves.iter().filter(|&mv| mv.from == from).collect();
        println!("Possible moves: {:?}", moves);
        let mut input = String::default();
        std::io::stdin().read_line(&mut input).expect("Worst");
        let Ok(to) = input.trim().parse::<usize>() else { continue };
        if let Some(&mv) = moves.iter().find(|mv| mv.to == to) {
            board.make_move(mv.clone());
            println!("{}", board);
        } else {
            println!("Wrong move");
        }
    }
}

fn run_all() {
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
