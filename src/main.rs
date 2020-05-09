use std::io;
use std::string::String;
use std::vec::Vec;

fn main() {
    let dim = get_board_size();
    let size = dim * dim;
    let mut board = vec![0u32; size as usize];

    draw_grid(&mut board, &dim);

    let mut count: u32 = 0;

    loop {
        count += 1;
        make_move(&mut board);
        draw_grid(&mut board, &dim);
        let have_winner = check_winner(&board, dim);
        if have_winner {
            println!("Winner winner chicken dinner!");
            break;
        }
        if &count == &(&dim * &dim) {
            break;
        }
    }
}

// TODO: seperate checkers to their own functions;
fn check_winner(board: &Vec<u32>, size: u32) -> bool {
    for i in 0..size {
        let mut matched_column = 0;
        let mut matched_row = 0;

        for j in 0..size {
            if board[(i * size + j) as usize] == 1 {
                matched_row += 1;
            }

            if board[(j * size + i) as usize] == 1 {
                matched_column += 1;
            }
        }

        if matched_row == 3 || matched_column == 3 {
            return true;
        }
    }

    for (index, value) in board.iter().enumerate() {
        let mut matched_diagnal = 0;
        let mut sum = index;
        if *value == 1 {
            for _i in 0..size {
                if board.len() - 1 >= sum && board[sum as usize] == 1 {
                    matched_diagnal += 1;
                }
                sum += (size + 1) as usize;
            }
        }

        if matched_diagnal == 3 {
            return true;
        }
    }

    false
}

fn make_move(board: &mut Vec<u32>) {
    println!("Please input your a position number that you want to check");

    let mut position = String::new();

    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");

    let position: u32 = position
        .trim()
        .parse()
        .expect("Error while parsing position value");

    board[(position - 1) as usize] = 1;
}

fn get_board_size() -> u32 {
    println!("Please input your a board size.");

    let mut grid_size = String::new();

    io::stdin()
        .read_line(&mut grid_size)
        .expect("Failed to read line");

    let token: u32 = grid_size.trim().parse().expect("It's not a number.");

    token
}

fn draw_grid(board: &Vec<u32>, row_boundary: &u32) {
    let mut count: u32 = 0;
    let mut grid = String::new();

    for value in board {
        if &count == row_boundary {
            count = 0;
            grid.push_str("\n")
        }

        let str = match value {
            1 => "x",
            2 => "o",
            _ => " ",
        };

        grid.push_str(format!("[{}]", str).as_str());
        count += 1;
    }

    println!("{}", grid);
}
