use std::io;
use std::string::String;
use std::vec::Vec;

fn main() {
    let dims = get_grid_size();
    let size = dims[0] * dims[1];
    let mut array = vec![0u32; size as usize];

    let grid = draw_grid(&mut array, &dims[0]);
    println!("{}", grid);

    let mut count: u32 = 0;

    loop {
        count += 1;
        check_tile(&mut array);
        let grid = draw_grid(&mut array, &dims[0]);
        check_best_available_position(&mut array, dims[0], dims[1]);
        println!("{}", grid);

        if &count == &(&dims[0] * &dims[1]) {
            break;
        }
    }
}

// TODO Computer player wip
fn check_best_available_position(array: &mut Vec<u32>, x_size: u32, y_size: u32) {
    for (i, value) in array.iter().enumerate() {
        for x in 0..x_size {
            println!("x = {}", x);
        }

        for y in 0..y_size {
            println!("y = {}", y);
        }
    }
}

fn check_tile(array: &mut Vec<u32>) {
    println!("Please input your a position number that you want to check");

    let mut position = String::new();

    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");

    let position: u32 = position
        .trim()
        .parse()
        .expect("Error while parsing position value");

    array[(position - 1) as usize] = 1;
}

fn draw_grid(array: &Vec<u32>, row_boundary: &u32) -> String {
    let mut count: u32 = 0;
    let mut grid = String::new();

    for value in array {
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

    grid
}

fn get_grid_size() -> Vec<u32> {
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
