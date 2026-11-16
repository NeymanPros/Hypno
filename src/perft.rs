use crate::{Board, Move};

pub fn perft(board: &mut Board, depth: u8) -> usize {
    if depth == 0 { return 0 };
    let mut amount = 0;
    board.generate_all_moves();
    if depth == 1 { return board.all_moves.len() };
    //amount += board.all_moves.len();
    for mv in &board.all_moves {
        let mut temp_board = board.clone();
        temp_board.make_move(mv.clone());
        amount += perft(&mut temp_board, depth - 1);
    }
    amount 
}