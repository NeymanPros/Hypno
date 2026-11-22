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
    let moves = perft::perft(&mut board, 6);
    println!("Moves: {}", moves);
    return;
}

fn run_some() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq - 0 1");    
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
    let mut board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", board);
    loop {
        let mut input = String::default();
        board.generate_all_moves();
        println!("\nMoves: {:?}\nChecks: {:?}\nXrays: {:?}", board.all_moves, board.checks, board.xrays);
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

fn run_uci() {
    let mut board = Board::default();
    println!("{}", board);
    loop {
        let mut input = String::default();
        board.generate_all_moves();
        std::io::stdin().read_line(&mut input).expect("Worst");
        let uci_mv = Move::from_uci(input);
        if let Some(mv) = board.all_moves.iter().find(|&mv| mv == &uci_mv) {
            board.make_move(mv.clone());
            println!("{}", board);
        } else {
            println!("Wrong move, mv: {uci_mv:?}");
        }
    }
}
