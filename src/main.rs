use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    X,
    I,
    O,
}

type Board = [[Move; 3]; 3];

fn main() {
    // TODO: win check func
    let mut board: Board = [[Move::I; 3]; 3];
    let mut current_move = Move::X;
    print_board(board);
    loop {
        let int_input = get_input();
        board = make_move(current_move, board, int_input);
        if current_move == Move::X {
            current_move = Move::O;
        } else if current_move == Move::O {
            current_move = Move::X
        }
        print_board(board);
    }
}

fn print_board(board: Board) {
    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] == Move::I {
                print!("{} ", y * 3 + x + 1);
            } else {
                print!("{:?} ", board[y][x]);
            }
        }
        println!();
    }
}

fn get_input() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let int_input = input
        .trim()
        .parse::<usize>()
        .expect("please give me corrent string number");
    // crushes when get string input (ex. onetwothree)
    return int_input;
}

fn make_move(current_move: Move, mut board: Board, mut input: usize) -> Board {
    while !(1..10).contains(&input) {
        println!("write number between 1 and 9");
        input = get_input();
    }
    let mut x = (input - 1) % 3;
    let mut y = (input - 1) / 3;
    while board[y][x] != Move::I {
        println!("cell filled");
        input = get_input();
        x = (input - 1) % 3;
        y = (input - 1) / 3;
    }
    board[(input - 1) / 3][(input - 1) % 3] = current_move;
    return board;
}
