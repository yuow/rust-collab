use std::io;
use std::io::*;

type Board = [[bool; 3]; 3];

fn main() {
    let mut board: Board = [[false, false, false]; 3];
    print_board(board);
    get_input();
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
