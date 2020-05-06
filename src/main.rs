use std::io;
use std::string::String;
use std::vec::Vec;

fn main() {
    let dims = get_board_size();
    let size = dims[0] * dims[1];
    let mut board = vec![0u32; size as usize];

    draw_grid(&mut board, &dims[0]);

    let mut count: u32 = 0;

    loop {
        count += 1;
        make_move(&mut board);
        draw_grid(&mut board, &dims[0]);

        if &count == &(&dims[0] * &dims[1]) {
            break;
        }
    }
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

fn get_board_size() -> Vec<u32> {
    println!("Please input your a grid size. For ex.: 3x3.");

    let mut grid_size = String::new();

    io::stdin()
        .read_line(&mut grid_size)
        .expect("Failed to read line");

    let tokens: Vec<u32> = grid_size
        .split("x")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    tokens
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
