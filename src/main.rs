use std::io;
use std::string::String;
use std::vec::Vec;

#[derive(Copy, Clone, PartialEq, Debug)]
enum SlotTypes {
    X,
    O,
    Empty,
}

fn main() {
    let dim = get_board_size();
    let size = dim * dim;
    let mut board: Vec<SlotTypes> = vec![SlotTypes::Empty; size as usize];

    draw_grid(&mut board, &dim);

    loop {
        make_move_human(&mut board, SlotTypes::X);
        make_smart_move(&mut board, dim, SlotTypes::O);
        draw_grid(&mut board, &dim);
        let winner = check_winner(&board, dim);
        if winner == SlotTypes::X {
            println!("You won!");
            break;
        } else if winner == SlotTypes::O {
            println!("You lost!");
            break;
        }

        let no_empty_slots = board.iter().all(|ref v| v == &&SlotTypes::Empty);

        if no_empty_slots {
            break;
        }
    }
}

// TODO: seperate checkers to their own functions;
fn check_winner(board: &Vec<SlotTypes>, size: u32) -> SlotTypes {
    for i in 0..size {
        let mut column_values = vec![SlotTypes::Empty; size as usize];
        let mut row_values = vec![SlotTypes::Empty; size as usize];

        for j in 0..size {
            row_values[j as usize] = board[(i * size + j) as usize];
            column_values[j as usize] = board[(j * size + i) as usize];
        }

        let row_contains_same_vals = row_values
            .iter()
            .all(|ref v| v == &&row_values[0] && v != &&SlotTypes::Empty);
        let columns_contains_same_vals = column_values
            .iter()
            .all(|ref v| v == &&column_values[0] && v != &&SlotTypes::Empty);

        if row_contains_same_vals {
            return row_values[0];
        }
        if columns_contains_same_vals {
            return column_values[0];
        }
    }

    for (index, value) in board.iter().enumerate() {
        let mut diagnal_values = vec![SlotTypes::Empty; size as usize];
        let mut sum = index;
        if *value != SlotTypes::Empty {
            for i in 0..size {
                if board.len() - 1 >= sum {
                    diagnal_values[i as usize] = board[sum as usize];
                }
                sum += (size + 1) as usize;
            }
        }

        let diagnal_contains_same_vals = diagnal_values
            .iter()
            .all(|ref v| v == &&diagnal_values[0] && v != &&SlotTypes::Empty);

        if diagnal_contains_same_vals {
            return diagnal_values[0];
        }
    }

    SlotTypes::Empty
}

fn minimax(
    board: &Vec<SlotTypes>,
    size: u32,
    is_maximizing: bool,
    player: SlotTypes,
) -> (i32, u32) {
    let mut winning_position = 0;
    if check_winner(&board, size) != SlotTypes::Empty {
        return (1, winning_position);
    }

    let mut board_clone = board.clone();
    let mut score = 0;
    for i in 0..board_clone.len() {
        println!("{:?} {}", board_clone[i], i);
        if board_clone[i] == SlotTypes::Empty {
            board_clone[i] = player;
            let (mut best_score, _) = minimax(
                &board_clone,
                size,
                if is_maximizing { false } else { true },
                player,
            );

            if !is_maximizing {
                best_score = -best_score;
            }

            if best_score > score {
                score = best_score;
                winning_position = i as u32;
            }
        }
    }

    (score, winning_position)
}

fn make_smart_move(board: &mut Vec<SlotTypes>, size: u32, player: SlotTypes) {
    let (_, pos) = minimax(&board, size, true, player);
    board[pos as usize] = player;
}

fn make_move_human(board: &mut Vec<SlotTypes>, player: SlotTypes) {
    println!("Please input your a position number that you want to check");

    let mut position = String::new();

    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");

    let position: u32 = position
        .trim()
        .parse()
        .expect("Error while parsing position value");

    board[(position - 1) as usize] = player;
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

fn draw_grid(board: &Vec<SlotTypes>, row_boundary: &u32) {
    let mut count: u32 = 0;
    let mut grid = String::new();

    for value in board {
        if &count == row_boundary {
            count = 0;
            grid.push_str("\n")
        }

        let str = match value {
            SlotTypes::X => "x",
            SlotTypes::O => "o",
            SlotTypes::Empty => " ",
        };

        grid.push_str(format!("[{}]", str).as_str());
        count += 1;
    }

    println!("{}", grid);
}
