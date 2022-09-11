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
        make_move(&mut current_move, &mut board);
        print_board(board);
        if check_win(board) {
            if current_move == Move::X {
                current_move = Move::O;
            } else if current_move == Move::O {
                current_move = Move::X;
            }
            println!("win {:?}", current_move);
            break;
        }
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
    println!("write number between 1 and 9");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let int_input = input.trim();

        match int_input.parse::<usize>() {
            Ok(i) => {
                if !(i < 1 || i > 9) {
                    return i - 1;
                }
            }
            _ => (),
        };
    }
}

fn make_move(current_move: &mut Move, board: &mut Board) {
    let mut input = get_input();
    let mut x = input % 3;
    let mut y = input / 3;

    while board[y][x] != Move::I {
        print_board(*board);
        println!("cell filled");
        input = get_input();
        x = input % 3;
        y = input / 3;
    }

    board[input / 3][input % 3] = *current_move;

    // idk why does dereferencing work this way
    if *current_move == Move::X {
        *current_move = Move::O;
    } else if *current_move == Move::O {
        *current_move = Move::X;
    }
}

fn check_win(board: Board) -> bool {
    for y in 0..3 {
        if board[y][0] == board[y][1] && board[y][1] == board[y][2] && board[y][0] != Move::I {
            return true;
        }
    }

    for x in 0..3 {
        if board[0][x] == board[1][x] && board[1][x] == board[2][x] && board[0][x] != Move::I {
            return true;
        }
    }

    if board[0][0] != Move::I && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return true;
    }

    if board[2][0] != Move::I && board[2][0] == board[1][1] && board[1][1] == board[0][2] {
        return true;
    }

    return false;
}
