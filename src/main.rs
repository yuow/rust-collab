use std::io;
use std::io::*;

#[derive(Debug, Clone, Copy)]
enum Move {
    X,
    I,
    O,
}

type Board = [[Move; 3]; 3];

fn main() {
    let mut board: Board = [[Move::I, Move::I, Move::I]; 3];
    print_board(board);
    get_input();
    board[0][0] = Move::X;
    board[0][1] = Move::O;
}

fn print_board(board: Board) {
    for i in 0..3 {
        println!("{:?}", board[i]);
    }
}

fn get_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("{}", input);
}
