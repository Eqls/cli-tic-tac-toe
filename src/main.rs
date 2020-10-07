use std::io;
use std::string::String;
use utils::SlotTypes;
use board::Board;

fn main() {
    let dim = get_board_size();
    let length = dim * dim;
    let board = Board {
        grid: vec![SlotTypes::Empty; length as usize],
        dim: get_board_size()
    };

    draw_grid(&board.grid, &dim);

    loop {
        make_move_human(&board, SlotTypes::X);
        let winner = check_winner(&board, dim);
        if winner == SlotTypes::X {
            draw_grid(&board, &dim);
            println!("You won!");
            break;
        } else if winner == SlotTypes::O {
            draw_grid(&mut board, &dim);
            println!("You lost!");
            break;
        }
        // make_move_smart(&mut board, dim, SlotTypes::O);
        draw_grid(&board, &dim);

        let no_empty_slots = board.iter().all(|ref v| v == &&SlotTypes::Empty);

        if no_empty_slots {
            break;
        }
    }
}

// fn opponent_for(player: SlotTypes) -> SlotTypes {
//     if player == SlotTypes::X {
//         return SlotTypes::X
//     }

//     return SlotTypes::O;
// }

// fn minimax(
//     board: &Vec<SlotTypes>,
//     size: u32,
//     is_maximizing: bool,
//     player: SlotTypes,
// ) -> (i32, u32) {
//     // let opponent_slot_type = if player == SlotTypes::X {
//     //     SlotTypes::O
//     // } else { SlotTypes::X };
//     let mut winning_position = 0;
//     if check_winner(&board, size) != SlotTypes::Empty {
//         return (1, winning_position);
//     }

//     let mut board_clone = board.clone();
//     let mut score = 0;
//     for i in 0..board_clone.len() {
//         // println!("{:?} {}", board_clone[i], i);
//         if board_clone[i] == SlotTypes::Empty {
//             board_clone[i] = player;
//             let (mut best_score, _) = minimax(
//                 &board_clone,
//                 size,
//                 if is_maximizing { false } else { true },
//                 if is_maximizing { player } else { opponent_for(player) },
//             );

//             if !is_maximizing {
//                 best_score = -best_score;
//             }

//             if best_score > score {
//                 score = best_score;
//                 winning_position = i as u32;
//             }
//         }
//     }

//     (score, winning_position)
// }

// fn make_move_smart(board: &Board, size: u32, player: SlotTypes) {
//     let (_, pos) = minimax(&board, size, true, player);
//     board.move(player, pos)
// }

fn make_move_human(board: &Board, player: SlotTypes) {
    println!("Please input your a position number that you want to check");

    let mut position = String::new();

    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");

    let position: u32 = position
        .trim()
        .parse()
        .expect("Error while parsing position value");

    board.make_move(player, position - 1)
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

fn draw_grid(board: &Board, row_boundary: &u32) {
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
